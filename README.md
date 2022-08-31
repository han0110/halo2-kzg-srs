# Halo2 KZG SRS

A tool to convert KZG SRS in other formats to [`pse/halo2`](https://github.com/privacy-scaling-explorations/halo2) format.

Currently supported and WIP formats are:

- [x] [Perpetual Powers of Tau](https://github.com/weijiekoh/perpetualpowersoftau)
- [x] [SnarkJS](https://github.com/iden3/snarkjs)
- [ ] [Aztec Ignition](https://github.com/AztecProtocol/Setup)

## Usage

### Perpetual Powers of Tau

To get SRS with `k = 10` from latest response of Perpetual Powers of Tau, we can run:

```shell
wget https://ppot.blob.core.windows.net/public/response_0071_edward
mkdir ./pse
cargo run --release --bin convert-from-perpetual-powers-of-tau response_0071_edward ./pse/srs- 10
```

Then it will output the SRS with `1 <= k <= 10` with path `./pse/srs-{k}`.

### SnarkJS

To get SRS with `k = 10` from Hermez's setup, we can run:

```shell
wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_10.ptau
mkdir ./pse
cargo run --release --bin convert-from-snarkjs powersOfTau28_hez_final_10.ptau ./pse/srs- 10
```

Then it will output the SRS with `1 <= k <= 10` with path `./pse/srs-{k}`.
