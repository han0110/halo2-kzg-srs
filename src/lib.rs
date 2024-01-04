use arithmetic::{g_to_lagrange, same_ratio};
use byteorder::{LittleEndian, ReadBytesExt};
use halo2_curves::{group::GroupEncoding, pairing::MultiMillerLoop, serde::SerdeObject};
use std::io;
use util::{perpetual_powers_of_tau, pse, snarkjs};

pub mod arithmetic;
pub mod util;

pub enum SrsFormat {
    /// From https://github.com/privacy-scaling-explorations/halo2
    Pse,
    /// From https://github.com/privacy-scaling-explorations/halo2
    PseRaw,
    /// From https://github.com/weijiekoh/perpetualpowersoftau
    PerpetualPowerOfTau(u32),
    /// From https://github.com/iden3/snarkjs
    SnarkJs,
}

#[derive(Clone, Debug, Eq)]
pub struct Srs<M: MultiMillerLoop> {
    pub k: u32,
    pub g: Vec<M::G1Affine>,
    pub g_lagrange: Vec<M::G1Affine>,
    pub g2: M::G2Affine,
    pub s_g2: M::G2Affine,
}

impl<M: MultiMillerLoop> PartialEq for Srs<M> {
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

impl<M: MultiMillerLoop> Srs<M>
where
    M::G1Affine: SerdeObject,
    M::G2Affine: SerdeObject,
{
    pub fn read<R: io::Read + io::Seek>(reader: &mut R, format: SrsFormat) -> Self {
        let desired_k = match format {
            SrsFormat::Pse | SrsFormat::PseRaw => reader.read_u32::<LittleEndian>().unwrap(),
            SrsFormat::PerpetualPowerOfTau(k) => k,
            SrsFormat::SnarkJs => snarkjs::read_k(reader),
        };
        reader.rewind().unwrap();
        Self::read_partial(reader, format, desired_k, true)
    }

    pub fn read_partial<R: io::Read + io::Seek>(
        reader: &mut R,
        format: SrsFormat,
        desired_k: u32,
        compute_lagrange: bool,
    ) -> Self {
        let srs = match format {
            SrsFormat::Pse => Self::read_partial_pse::<_, false>(reader, desired_k),
            SrsFormat::PseRaw => Self::read_partial_pse::<_, true>(reader, desired_k),
            SrsFormat::PerpetualPowerOfTau(k) => {
                assert!(desired_k <= k);

                let n = 1 << desired_k;

                let g = perpetual_powers_of_tau::read_g1s::<M, _, false>(reader, n);
                let g_lagrange = if compute_lagrange {
                    g_to_lagrange(&g, desired_k)
                } else {
                    vec![]
                };

                let [g2, s_g2]: [_; 2] =
                    perpetual_powers_of_tau::read_g2s::<M, _, false>(reader, k, 2)
                        .try_into()
                        .unwrap();

                Self {
                    k: desired_k,
                    g,
                    g_lagrange,
                    g2,
                    s_g2,
                }
            }
            SrsFormat::SnarkJs => {
                let k = snarkjs::read_k(reader);
                assert!(desired_k <= k);

                let n = 1 << desired_k;

                let g = snarkjs::read_g1s::<M, _, false>(reader, n);
                let g_lagrange = g_to_lagrange(&g, desired_k);

                let [g2, s_g2]: [_; 2] = snarkjs::read_g2s::<M, _, false>(reader, 2)
                    .try_into()
                    .unwrap();

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

    fn read_partial_pse<R: io::Read + io::Seek, const RAW: bool>(
        reader: &mut R,
        desired_k: u32,
    ) -> Self {
        let k = pse::read_k(reader);
        assert!(desired_k <= k);

        let n = 1 << desired_k;

        let g = pse::read_g1s::<M, _, RAW, false>(reader, n);
        let g_lagrange = if k == desired_k {
            pse::read_g1s::<M, _, RAW, true>(reader, n)
        } else {
            g_to_lagrange(&g, desired_k)
        };

        let [g2, s_g2]: [_; 2] = pse::read_g2s::<M, _, RAW, false>(reader, 2)
            .try_into()
            .unwrap();

        Self {
            k: desired_k,
            g,
            g_lagrange,
            g2,
            s_g2,
        }
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

    pub fn write_raw(&self, writer: &mut impl io::Write) {
        writer.write_all(&self.k.to_le_bytes()).unwrap();
        for point in self.g.iter() {
            point.write_raw(writer).unwrap();
        }
        for point in self.g_lagrange.iter() {
            point.write_raw(writer).unwrap();
        }
        self.g2.write_raw(writer).unwrap();
        self.s_g2.write_raw(writer).unwrap();
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
        same_ratio::<M>(&self.g, self.g2, self.s_g2)
    }
}

#[cfg(test)]
mod test {
    use super::{Srs, SrsFormat};
    use halo2_curves::bn256::Bn256;
    use std::{fs::File, io::Cursor};

    #[test]
    fn test_perpetual_powers_of_tau() {
        const PATH: &str = "./src/testdata/perpetual-powers-of-tau/bn254-8";
        let from_perpetual_powers_of_tau = Srs::<Bn256>::read(
            &mut File::open(PATH).unwrap(),
            SrsFormat::PerpetualPowerOfTau(8),
        );
        let from_pse = {
            let mut buf = Vec::new();
            from_perpetual_powers_of_tau.write(&mut buf);
            Srs::<Bn256>::read(&mut Cursor::new(buf), SrsFormat::Pse)
        };
        assert_eq!(from_perpetual_powers_of_tau, from_pse);
    }

    #[test]
    fn test_snarkjs() {
        const PATH: &str = "./src/testdata/snarkjs/bn254-8";
        let from_snarkjs = Srs::<Bn256>::read(&mut File::open(PATH).unwrap(), SrsFormat::SnarkJs);
        let from_pse = {
            let mut buf = Vec::new();
            from_snarkjs.write(&mut buf);
            Srs::<Bn256>::read(&mut Cursor::new(buf), SrsFormat::Pse)
        };
        assert_eq!(from_snarkjs, from_pse);
    }
}
