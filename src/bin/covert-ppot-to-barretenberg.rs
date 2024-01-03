use halo2_curves::bn256::Bn256;
use halo2_kzg_srs::{Srs, SrsFormat};
use std::{env, fs::File};
use std::io::Write;
use halo2_curves::serde::SerdeObject;

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
        false,
    );

    let g_bytes = srs.g.into_iter().flat_map(|point| point.to_raw_bytes()).collect::<Vec<u8>>();
    let path = format!("{dst_prefix}-{desired_k}.g1");
    let mut file = File::create(path).expect("Couldn't create file at {path}");
    file.write_all(&g_bytes).expect("Failed to write g1 points to file");

    let path = format!("{dst_prefix}-{desired_k}.g2");
    let mut file = File::create(path).expect("Couldn't create file at {path}");
    file.write_all(&srs.g2.to_raw_bytes()).expect("Failed to write g2 points to file");
}
