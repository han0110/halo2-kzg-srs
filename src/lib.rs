use arithmetic::{best_multiexp, g_to_lagrange, parallelize, CurveRead};
use byteorder::{LittleEndian, ReadBytesExt};
use halo2_curves::{
    group::{
        ff::{Field, PrimeField},
        Group, GroupEncoding,
    },
    pairing::{MillerLoopResult, MultiMillerLoop},
    CurveAffine, FieldExt,
};
use num_bigint::BigUint;
use rand_core::OsRng;
use std::{io, iter};

mod arithmetic;

pub enum SrsFormat {
    /// From https://github.com/privacy-scaling-explorations/halo2
    Pse,
    /// From https://github.com/weijiekoh/perpetualpowersoftau
    PerpetualPowerOfTau(u32),
    /// From https://github.com/iden3/snarkjs
    SnarkJs,
}

#[derive(Clone, Debug, Eq)]
pub struct Srs<E: MultiMillerLoop> {
    pub k: u32,
    pub g: Vec<E::G1Affine>,
    pub g_lagrange: Vec<E::G1Affine>,
    pub g2: E::G2Affine,
    pub s_g2: E::G2Affine,
}

impl<E: MultiMillerLoop> PartialEq for Srs<E> {
    fn eq(&self, other: &Self) -> bool {
        (&self.k, &self.g, &self.g_lagrange, &self.g2, &self.s_g2).eq(&(
            &other.k,
            &other.g,
            &other.g_lagrange,
            &other.g2,
            &other.s_g2,
        ))
    }
}

impl<E: MultiMillerLoop> Srs<E> {
    pub fn read<R: io::Read + io::Seek>(reader: &mut R, format: SrsFormat) -> Self {
        let desired_k = match format {
            SrsFormat::Pse => reader.read_u32::<LittleEndian>().unwrap(),
            SrsFormat::PerpetualPowerOfTau(k) => k,
            SrsFormat::SnarkJs => {
                let header_size = {
                    reader.seek(io::SeekFrom::Start(16)).unwrap();
                    reader.read_u64::<LittleEndian>().unwrap()
                };

                let k_offset = 24 + header_size - 8;
                reader.seek(io::SeekFrom::Start(k_offset)).unwrap();
                reader.read_u32::<LittleEndian>().unwrap()
            }
        };
        reader.rewind().unwrap();
        Self::read_partial(reader, format, desired_k)
    }

    pub fn read_partial<R: io::Read + io::Seek>(
        reader: &mut R,
        format: SrsFormat,
        desired_k: u32,
    ) -> Self {
        let srs = match format {
            SrsFormat::Pse => {
                let k = reader.read_u32::<LittleEndian>().unwrap();
                assert!(desired_k <= k);

                let n = 1 << desired_k;

                let read_points = |reader: &mut R| {
                    let mut reprs = vec![<E::G1Affine as GroupEncoding>::Repr::default(); n];
                    for repr in reprs.iter_mut() {
                        reader.read_exact(repr.as_mut()).unwrap();
                    }

                    let mut points = vec![E::G1Affine::default(); n];
                    parallelize(&mut points, |points, chunks| {
                        for (i, point) in points.iter_mut().enumerate() {
                            *point = E::G1Affine::from_bytes(&reprs[chunks + i]).unwrap();
                        }
                    });
                    points
                };

                let g = read_points(reader);
                let g_lagrange = if k == desired_k {
                    read_points(reader)
                } else {
                    g_to_lagrange(&g, desired_k)
                };

                let g1_size = curve_repr_size::<E::G1Affine>();
                let g2_offset = 4 + g1_size * 2 * (1 << k);
                reader.seek(io::SeekFrom::Start(g2_offset as u64)).unwrap();
                let g2 = E::G2Affine::read(reader).unwrap();
                let s_g2 = E::G2Affine::read(reader).unwrap();

                Self {
                    k: desired_k,
                    g,
                    g_lagrange,
                    g2,
                    s_g2,
                }
            }
            SrsFormat::PerpetualPowerOfTau(k) => {
                assert!(desired_k <= k);

                fn read_points<C: CurveAffine, R: io::Read>(reader: &mut R, n: usize) -> Vec<C> {
                    let mut reprs = vec![C::Repr::default(); n];
                    for repr in reprs.iter_mut() {
                        reader.read_exact(repr.as_mut()).unwrap();
                        repr.as_mut().reverse();
                    }

                    let mut points = vec![C::default(); n];
                    parallelize(&mut points, |points, chunks| {
                        for (i, point) in points.iter_mut().enumerate() {
                            let candidate = C::from_bytes(&reprs[chunks + i]).unwrap();
                            let minus_candidate = -candidate;

                            *point = if (candidate.coordinates().unwrap().y()
                                < minus_candidate.coordinates().unwrap().y())
                                ^ ((reprs[chunks + i].as_ref().last().unwrap() & 0b1000_0000) != 0)
                            {
                                candidate
                            } else {
                                minus_candidate
                            }
                        }
                    });
                    points
                }

                let n = 1 << desired_k;

                reader.seek(io::SeekFrom::Start(64)).unwrap();
                let g = read_points::<E::G1Affine, _>(reader, n);
                let g_lagrange = g_to_lagrange(&g, desired_k);

                let g1_size = curve_repr_size::<E::G1Affine>();
                let g2_offset = 64 + g1_size * (2 * (1 << k) - 1);
                reader.seek(io::SeekFrom::Start(g2_offset as u64)).unwrap();
                let g2 = read_points(reader, 1)[0];
                let s_g2 = read_points(reader, 1)[0];

                Self {
                    k: desired_k,
                    g,
                    g_lagrange,
                    g2,
                    s_g2,
                }
            }
            SrsFormat::SnarkJs => {
                let header_size = {
                    reader.seek(io::SeekFrom::Start(16)).unwrap();
                    reader.read_u64::<LittleEndian>().unwrap()
                };
                let k = {
                    let k_offset = 24 + header_size - 8;
                    reader.seek(io::SeekFrom::Start(k_offset)).unwrap();
                    reader.read_u32::<LittleEndian>().unwrap()
                };
                assert!(desired_k <= k);

                let n = 1 << desired_k;

                fn read_g1_points<G1: CurveAffine, R: io::Read>(
                    reader: &mut R,
                    n: usize,
                ) -> Vec<G1> {
                    let mut reprs = vec![<G1::Base as PrimeField>::Repr::default(); 2 * n];
                    for repr in reprs.iter_mut() {
                        reader.read_exact(repr.as_mut()).unwrap();
                    }

                    let mont_r_inv = mont_r::<G1::Base>().invert().unwrap();
                    let mut points = vec![G1::default(); n];
                    parallelize(&mut points, |points, chunks| {
                        for (i, point) in points.iter_mut().enumerate() {
                            let x =
                                G1::Base::from_repr(reprs[2 * (chunks + i)]).unwrap() * mont_r_inv;
                            let y = G1::Base::from_repr(reprs[2 * (chunks + i) + 1]).unwrap()
                                * mont_r_inv;
                            *point = G1::from_xy(x, y).unwrap();
                        }
                    });
                    points
                }

                fn read_g2_point<G1: CurveAffine, G2: CurveAffine, R: io::Read>(
                    reader: &mut R,
                ) -> G2 {
                    let mut reprs = [<G2::Base as PrimeField>::Repr::default(); 2];
                    for repr in reprs.iter_mut() {
                        reader.read_exact(repr.as_mut()).unwrap();
                    }

                    let mont_r_inv = mont_r::<G1::Base>().invert().unwrap();
                    for repr in reprs.iter_mut() {
                        let g1_base_size = field_repr_size::<G1::Base>();
                        let mut g1_base_reprs = [<G1::Base as PrimeField>::Repr::default(); 2];
                        g1_base_reprs[0]
                            .as_mut()
                            .copy_from_slice(&repr.as_ref()[..g1_base_size]);
                        g1_base_reprs[1]
                            .as_mut()
                            .copy_from_slice(&repr.as_ref()[g1_base_size..]);
                        let g1_bases = g1_base_reprs.map(|g1_base_repr| {
                            G1::Base::from_repr(g1_base_repr).unwrap() * mont_r_inv
                        });
                        repr.as_mut()[..g1_base_size]
                            .copy_from_slice(g1_bases[0].to_repr().as_ref());
                        repr.as_mut()[g1_base_size..]
                            .copy_from_slice(g1_bases[1].to_repr().as_ref());
                    }

                    let [x, y] = reprs.map(|repr| G2::Base::from_repr(repr).unwrap());
                    G2::from_xy(x, y).unwrap()
                }

                let g1_offset = 24 + header_size + 12;
                reader.seek(io::SeekFrom::Start(g1_offset)).unwrap();
                let g = read_g1_points::<E::G1Affine, _>(reader, n);
                let g_lagrange = g_to_lagrange(&g, desired_k);

                let g1_base_size = field_repr_size::<<E::G1Affine as CurveAffine>::Base>();
                let g2_offset = g1_offset + (2 * g1_base_size * (2 * (1 << k) - 1)) as u64 + 12;
                reader.seek(io::SeekFrom::Start(g2_offset)).unwrap();
                let g2 = read_g2_point::<E::G1Affine, E::G2Affine, _>(reader);
                let s_g2 = read_g2_point::<E::G1Affine, E::G2Affine, _>(reader);

                Self {
                    k: desired_k,
                    g,
                    g_lagrange,
                    g2,
                    s_g2,
                }
            }
        };

        assert!(srs.validate());

        srs
    }

    pub fn write(&self, writer: &mut impl io::Write) {
        writer.write_all(&self.k.to_le_bytes()).unwrap();
        for point in self.g.iter() {
            writer.write_all(point.to_bytes().as_ref()).unwrap();
        }
        for point in self.g_lagrange.iter() {
            writer.write_all(point.to_bytes().as_ref()).unwrap();
        }
        writer.write_all(self.g2.to_bytes().as_ref()).unwrap();
        writer.write_all(self.s_g2.to_bytes().as_ref()).unwrap();
    }

    pub fn downsize(&mut self, k: u32) {
        assert!(k <= self.k);

        if k == self.k {
            return;
        }

        let n = 1 << k;

        self.k = k;
        self.g.truncate(n as usize);
        self.g_lagrange = g_to_lagrange(&self.g, k);
    }

    fn validate(&self) -> bool {
        let coeffs = iter::repeat_with(|| E::Scalar::random(OsRng))
            .take(self.g.len() - 1)
            .collect::<Vec<_>>();

        let lhs = best_multiexp(&coeffs, &self.g[..self.g.len() - 1]);
        let rhs = best_multiexp(&coeffs, &self.g[1..self.g.len()]);

        E::multi_miller_loop(&[
            (&lhs.into(), &self.s_g2.into()),
            (&rhs.into(), &(-self.g2).into()),
        ])
        .final_exponentiation()
        .is_identity()
        .into()
    }
}

fn field_repr_size<F: PrimeField>() -> usize {
    F::Repr::default().as_ref().len()
}

fn curve_repr_size<C: CurveAffine>() -> usize {
    C::Repr::default().as_ref().len()
}

fn modulus<F: FieldExt>() -> BigUint {
    BigUint::from_bytes_le((-F::one()).to_repr().as_ref()) + 1u64
}

fn mont_r<F: FieldExt>() -> F {
    let mut repr = F::Repr::default();
    let mont_r = (BigUint::from(1u64) << (8 * field_repr_size::<F>())) % modulus::<F>();
    repr.as_mut().copy_from_slice(&mont_r.to_bytes_le());
    F::from_repr(repr).unwrap()
}

#[cfg(test)]
mod test {
    use super::{Srs, SrsFormat};
    use halo2_curves::bn256::Bn256;
    use std::{fs::File, io::Cursor};

    #[test]
    fn test_perpetual_powers_of_tau() {
        const PATH: &str = "./src/fixture/perpetual-powers-of-tau/bn254-8";
        let from_pot = Srs::<Bn256>::read(
            &mut File::open(PATH).unwrap(),
            SrsFormat::PerpetualPowerOfTau(8),
        );
        let from_pse = {
            let mut buf = Vec::new();
            from_pot.write(&mut buf);
            Srs::<Bn256>::read(&mut Cursor::new(buf), SrsFormat::Pse)
        };
        assert_eq!(from_pot, from_pse);
    }

    #[test]
    fn test_snarkjs() {
        const PATH: &str = "./src/fixture/snarkjs/bn254-8";
        let from_snarkjs = Srs::<Bn256>::read(&mut File::open(PATH).unwrap(), SrsFormat::SnarkJs);
        let from_pse = {
            let mut buf = Vec::new();
            from_snarkjs.write(&mut buf);
            Srs::<Bn256>::read(&mut Cursor::new(buf), SrsFormat::Pse)
        };
        assert_eq!(from_snarkjs, from_pse);
    }
}
