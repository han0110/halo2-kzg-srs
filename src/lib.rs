use arithmetic::{best_multiexp, g_to_lagrange, parallelize, CurveRead};
use halo2_curves::{
    group::{ff::Field, prime::PrimeCurveAffine, Group, GroupEncoding},
    pairing::{MillerLoopResult, MultiMillerLoop},
    CurveAffine,
};
use rand_core::OsRng;
use std::{io, iter};

mod arithmetic;

pub enum SrsFormat {
    /// From https://github.com/privacy-scaling-explorations/halo2
    Pse,
    /// From https://github.com/weijiekoh/perpetualpowersoftau
    PerpetualPowerOfTau(u32),
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
            SrsFormat::Pse => {
                let mut bytes = [0u8; 4];
                reader.read_exact(&mut bytes[..]).unwrap();
                reader.rewind().unwrap();
                u32::from_le_bytes(bytes)
            }
            SrsFormat::PerpetualPowerOfTau(k) => k,
        };
        Self::read_partial(reader, format, desired_k)
    }

    pub fn read_partial<R: io::Read + io::Seek>(
        reader: &mut R,
        format: SrsFormat,
        desired_k: u32,
    ) -> Self {
        let srs = match format {
            SrsFormat::Pse => {
                let k = {
                    let mut bytes = [0u8; 4];
                    reader.read_exact(&mut bytes[..]).unwrap();
                    u32::from_le_bytes(bytes)
                };
                assert!(desired_k <= k);

                let n = 1 << desired_k;

                let read_points = |reader: &mut R| -> Vec<E::G1Affine> {
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
                    let g_projective = g.iter().map(|g| g.to_curve()).collect::<Vec<_>>();
                    g_to_lagrange(g_projective, desired_k)
                };

                let g1_size = repr_size::<E::G1Affine>();
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

                fn read_points<C: CurveAffine>(reader: &mut impl io::Read, n: usize) -> Vec<C> {
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
                let g = read_points::<E::G1Affine>(reader, n);
                let g_projective = g.iter().map(|g| g.to_curve()).collect::<Vec<_>>();
                let g_lagrange = g_to_lagrange(g_projective, desired_k);

                let g1_size = repr_size::<E::G1Affine>();
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
        self.g_lagrange = g_to_lagrange(self.g.iter().map(|g| g.to_curve()).collect(), k);
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

fn repr_size<C: CurveAffine>() -> usize {
    C::Repr::default().as_mut().len()
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
}
