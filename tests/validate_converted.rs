use halo2_curves::bn256::{Bn256, Fq, G1Affine, G2Affine};
use halo2_kzg_srs::{
    arithmetic::same_ratio,
    util::{ec_point_repr_size, field_repr_size, perpetual_powers_of_tau, pse, snarkjs},
};
use hyper::{body::HttpBody, Body, Client, Request};
use hyper_tls::HttpsConnector;
use std::io::Cursor;

async fn fetch(uri: &str, offset: usize, length: usize) -> Vec<u8> {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let req = Request::get(uri)
        .header("Range", format!("bytes={offset}-{}", offset + length))
        .body(Body::default())
        .unwrap();
    let mut resp = client.request(req).await.unwrap();
    assert!(resp.status().is_success());

    let mut buf = Vec::new();
    while let Some(chunk) = resp.body_mut().data().await {
        buf.extend(chunk.unwrap());
        if buf.len() > length {
            break;
        }
    }
    buf.drain(..length).collect()
}

async fn fetch_pse_g1s(source: &str, k: usize, max_k: usize) -> Vec<G1Affine> {
    let uri = format!("https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/{source}-{k}");
    let length = pse::G1_OFFSET as usize + (ec_point_repr_size::<G1Affine>() << k.min(max_k));
    let mut reader = Cursor::new(fetch(&uri, 0, length).await);
    pse::read_g1s::<Bn256, _, false>(&mut reader, k.min(max_k))
}

async fn fetch_hermez_g2s() -> [G2Affine; 2] {
    let uri = "https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_08.ptau";
    let header_size = {
        let mut reader = Cursor::new(fetch(uri, 0, snarkjs::HEADER_OFFSET as usize).await);
        snarkjs::read_header_size(&mut reader)
    };
    let g2_offset = {
        let mut reader =
            Cursor::new(fetch(uri, 0, (snarkjs::HEADER_OFFSET + header_size) as usize).await);
        snarkjs::read_g2_offset::<Bn256, _>(&mut reader)
    };
    let mut reader = Cursor::new(fetch(uri, g2_offset as usize, 8 * field_repr_size::<Fq>()).await);
    snarkjs::read_g2s::<Bn256, _, true>(&mut reader, 2)
        .try_into()
        .unwrap()
}

async fn fetch_perpetual_powers_of_tau_g2() -> [G2Affine; 2] {
    const K: u32 = 28;
    let uri = "https://ppot.blob.core.windows.net/public/response_0071_edward";
    let g2_offset = perpetual_powers_of_tau::g2_offset::<Bn256>(K) as usize;
    let mut reader = Cursor::new(fetch(uri, g2_offset, 2 * ec_point_repr_size::<G2Affine>()).await);
    perpetual_powers_of_tau::read_g2s::<Bn256, _, true>(&mut reader, K, 2)
        .try_into()
        .unwrap()
}

#[tokio::test]
async fn validate_converted_hermez() {
    let [g2, s_g2] = fetch_hermez_g2s().await;
    for k in 1..=26 {
        let g1s = fetch_pse_g1s("hermez", k, 16).await;
        assert!(same_ratio::<Bn256>(&g1s, g2, s_g2));
    }
}

#[tokio::test]
async fn validate_converted_perpetual_powers_of_tau() {
    let [g2, s_g2] = fetch_perpetual_powers_of_tau_g2().await;
    for k in 1..=26 {
        let g1s = fetch_pse_g1s("perpetual-powers-of-tau", k, 16).await;
        assert!(same_ratio::<Bn256>(&g1s, g2, s_g2));
    }
}
