use halo2_curves::bn256::Bn256;
use halo2_kzg_srs::{Srs, SrsFormat};
use std::{env, fs::File};

fn main() {
    let src = env::args()
        .nth(1)
        .expect("Please specify source file path to convert");
    let dst_prefix = env::args()
        .nth(2)
        .expect("Please specify destination file path prefix (will be appended with suffix k)");
    let desired_k = env::args()
        .nth(3)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(28);

    let srs = Srs::<Bn256>::read_partial(
        &mut File::open(src).expect("Couldn't open file at {path}"),
        SrsFormat::PerpetualPowerOfTau(28),
        desired_k,
    );

    for k in (1..=srs.k).rev() {
        let mut srs = srs.clone();
        srs.downsize(k);
        let path = format!("{dst_prefix}{k}");
        srs.write_raw(&mut File::create(path).expect("Couldn't create file at {path}"))
    }
}
