use halo2_curves::{group::ff::PrimeField, CurveAffine, FieldExt};
use num_bigint::BigUint;

pub fn field_repr_size<F: PrimeField>() -> usize {
    F::Repr::default().as_ref().len()
}

pub fn ec_point_repr_size<C: CurveAffine>() -> usize {
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

pub mod pse {
    use crate::{arithmetic::parallelize, util::ec_point_repr_size};
    use byteorder::{LittleEndian, ReadBytesExt};
    use halo2_curves::{pairing::MultiMillerLoop, CurveAffine};
    use std::io;

    pub const G1_OFFSET: u64 = 4;

    pub fn read_k<R: io::Read + io::Seek>(reader: &mut R) -> u32 {
        reader.seek(io::SeekFrom::Start(0)).unwrap();
        reader.read_u32::<LittleEndian>().unwrap()
    }

    fn seek_g1_offset<R: io::Read + io::Seek>(reader: &mut R) {
        reader.seek(io::SeekFrom::Start(G1_OFFSET)).unwrap();
    }

    fn seek_g2_offset<M: MultiMillerLoop, R: io::Read + io::Seek>(reader: &mut R) {
        let g1_size = ec_point_repr_size::<M::G1Affine>();
        let offset = 4 + g1_size * 2 * (1 << read_k(reader));
        reader.seek(io::SeekFrom::Start(offset as u64)).unwrap();
    }

    fn read_ec_points<C: CurveAffine, R: io::Read>(reader: &mut R, n: usize) -> Vec<C> {
        let mut reprs = vec![C::Repr::default(); n];
        for repr in reprs.iter_mut() {
            reader.read_exact(repr.as_mut()).unwrap();
        }

        let mut points = vec![C::default(); n];
        parallelize(&mut points, |points, start| {
            for (i, point) in points.iter_mut().enumerate() {
                *point = C::from_bytes(&reprs[start + i]).unwrap();
            }
        });
        points
    }

    pub fn read_g1s<M: MultiMillerLoop, R: io::Read + io::Seek, const IN_PLACE: bool>(
        reader: &mut R,
        n: usize,
    ) -> Vec<M::G1Affine> {
        if !IN_PLACE {
            seek_g1_offset(reader);
        }
        read_ec_points(reader, n)
    }

    pub fn read_g2s<M: MultiMillerLoop, R: io::Read + io::Seek, const IN_PLACE: bool>(
        reader: &mut R,
        n: usize,
    ) -> Vec<M::G2Affine> {
        if !IN_PLACE {
            seek_g2_offset::<M, _>(reader);
        }
        read_ec_points(reader, n)
    }
}

pub mod perpetual_powers_of_tau {
    use crate::{arithmetic::parallelize, util::ec_point_repr_size};
    use halo2_curves::{pairing::MultiMillerLoop, CurveAffine};
    use std::io;

    pub const G1_OFFSET: u64 = 64;

    fn seek_g1_offset<R: io::Read + io::Seek>(reader: &mut R) {
        reader.seek(io::SeekFrom::Start(G1_OFFSET)).unwrap();
    }

    pub fn g2_offset<M: MultiMillerLoop>(k: u32) -> u64 {
        let g1_size = ec_point_repr_size::<M::G1Affine>() as u64;
        G1_OFFSET + g1_size * (2 * (1 << k) - 1)
    }

    fn seek_g2_offset<M: MultiMillerLoop, R: io::Read + io::Seek>(reader: &mut R, k: u32) {
        let offset = g2_offset::<M>(k);
        reader.seek(io::SeekFrom::Start(offset)).unwrap();
    }

    fn read_ec_points<C: CurveAffine, R: io::Read + io::Seek>(reader: &mut R, n: usize) -> Vec<C> {
        let mut reprs = vec![C::Repr::default(); n];
        for repr in reprs.iter_mut() {
            reader.read_exact(repr.as_mut()).unwrap();
            repr.as_mut().reverse();
        }

        let mut points = vec![C::default(); n];
        parallelize(&mut points, |points, start| {
            for (i, point) in points.iter_mut().enumerate() {
                let candidate = C::from_bytes(&reprs[start + i]).unwrap();
                let minus_candidate = -candidate;

                *point = if (candidate.coordinates().unwrap().y()
                    < minus_candidate.coordinates().unwrap().y())
                    ^ ((reprs[start + i].as_ref().last().unwrap() & 0b1000_0000) != 0)
                {
                    candidate
                } else {
                    minus_candidate
                }
            }
        });
        points
    }

    pub fn read_g1s<M: MultiMillerLoop, R: io::Read + io::Seek, const IN_PLACE: bool>(
        reader: &mut R,
        n: usize,
    ) -> Vec<M::G1Affine> {
        if !IN_PLACE {
            seek_g1_offset(reader);
        }
        read_ec_points::<M::G1Affine, _>(reader, n)
    }

    pub fn read_g2s<M: MultiMillerLoop, R: io::Read + io::Seek, const IN_PLACE: bool>(
        reader: &mut R,
        k: u32,
        n: usize,
    ) -> Vec<M::G2Affine> {
        if !IN_PLACE {
            seek_g2_offset::<M, _>(reader, k);
        }
        read_ec_points::<M::G2Affine, _>(reader, n)
    }
}

pub mod snarkjs {
    use crate::{
        arithmetic::parallelize,
        util::{field_repr_size, mont_r},
    };
    use byteorder::{LittleEndian, ReadBytesExt};
    use halo2_curves::{
        group::ff::{Field, PrimeField},
        pairing::MultiMillerLoop,
        CurveAffine,
    };
    use std::io;

    pub const HEADER_SIZE_OFFSET: u64 = 16;
    pub const HEADER_OFFSET: u64 = HEADER_SIZE_OFFSET + 8;

    pub fn read_header_size<R: io::Read + io::Seek>(reader: &mut R) -> u64 {
        reader
            .seek(io::SeekFrom::Start(HEADER_SIZE_OFFSET))
            .unwrap();
        reader.read_u64::<LittleEndian>().unwrap()
    }

    pub fn read_k<R: io::Read + io::Seek>(reader: &mut R) -> u32 {
        let k_offset = HEADER_OFFSET + read_header_size(reader) - 8;
        reader.seek(io::SeekFrom::Start(k_offset)).unwrap();
        reader.read_u32::<LittleEndian>().unwrap()
    }

    pub fn read_g1_offset<R: io::Read + io::Seek>(reader: &mut R) -> u64 {
        HEADER_OFFSET + read_header_size(reader) + 12
    }

    pub fn read_g2_offset<M: MultiMillerLoop, R: io::Read + io::Seek>(reader: &mut R) -> u64 {
        let base_size = field_repr_size::<<M::G1Affine as CurveAffine>::Base>();
        read_g1_offset(reader) + (2 * base_size * (2 * (1 << read_k(reader)) - 1)) as u64 + 12
    }

    fn seek_g1_offset<R: io::Read + io::Seek>(reader: &mut R) {
        let offset = read_g1_offset(reader);
        reader.seek(io::SeekFrom::Start(offset)).unwrap();
    }

    fn seek_g2_offset<M: MultiMillerLoop, R: io::Read + io::Seek>(reader: &mut R) {
        let offset = read_g2_offset::<M, _>(reader);
        reader.seek(io::SeekFrom::Start(offset)).unwrap();
    }

    pub fn read_g1s<M: MultiMillerLoop, R: io::Read + io::Seek, const IN_PLACE: bool>(
        reader: &mut R,
        n: usize,
    ) -> Vec<M::G1Affine> {
        if !IN_PLACE {
            seek_g1_offset(reader);
        }

        let mut reprs =
            vec![<<M::G1Affine as CurveAffine>::Base as PrimeField>::Repr::default(); 2 * n];
        for repr in reprs.iter_mut() {
            reader.read_exact(repr.as_mut()).unwrap();
        }

        let mont_r_inv = mont_r::<<M::G1Affine as CurveAffine>::Base>()
            .invert()
            .unwrap();
        let mut points = vec![M::G1Affine::default(); n];
        parallelize(&mut points, |points, start| {
            for (i, point) in points.iter_mut().enumerate() {
                let x = <M::G1Affine as CurveAffine>::Base::from_repr(reprs[2 * (start + i)])
                    .unwrap()
                    * mont_r_inv;
                let y = <M::G1Affine as CurveAffine>::Base::from_repr(reprs[2 * (start + i) + 1])
                    .unwrap()
                    * mont_r_inv;
                *point = M::G1Affine::from_xy(x, y).unwrap();
            }
        });
        points
    }

    pub fn read_g2s<M: MultiMillerLoop, R: io::Read + io::Seek, const IN_PLACE: bool>(
        reader: &mut R,
        n: usize,
    ) -> Vec<M::G2Affine> {
        if !IN_PLACE {
            seek_g2_offset::<M, _>(reader);
        }

        let mut reprs =
            vec![[<<M::G2Affine as CurveAffine>::Base as PrimeField>::Repr::default(); 2]; n];
        for repr in reprs.iter_mut() {
            reader.read_exact(repr[0].as_mut()).unwrap();
            reader.read_exact(repr[1].as_mut()).unwrap();
        }

        let g1_base_size = field_repr_size::<<M::G1Affine as CurveAffine>::Base>();
        let mont_r_inv = mont_r::<<M::G1Affine as CurveAffine>::Base>()
            .invert()
            .unwrap();
        let mut points = vec![M::G2Affine::default(); n];
        parallelize(&mut points, |points, start| {
            for (i, point) in points.iter_mut().enumerate() {
                let reprs = reprs[start + i].map(|mut repr| {
                    let mut g1_base_reprs =
                        [<<M::G1Affine as CurveAffine>::Base as PrimeField>::Repr::default(); 2];
                    g1_base_reprs[0]
                        .as_mut()
                        .copy_from_slice(&repr.as_ref()[..g1_base_size]);
                    g1_base_reprs[1]
                        .as_mut()
                        .copy_from_slice(&repr.as_ref()[g1_base_size..]);
                    let g1_bases = g1_base_reprs.map(|g1_base_repr| {
                        <M::G1Affine as CurveAffine>::Base::from_repr(g1_base_repr).unwrap()
                            * mont_r_inv
                    });
                    repr.as_mut()[..g1_base_size].copy_from_slice(g1_bases[0].to_repr().as_ref());
                    repr.as_mut()[g1_base_size..].copy_from_slice(g1_bases[1].to_repr().as_ref());
                    repr
                });
                let [x, y] =
                    reprs.map(|repr| <M::G2Affine as CurveAffine>::Base::from_repr(repr).unwrap());
                *point = M::G2Affine::from_xy(x, y).unwrap();
            }
        });
        points
    }
}
