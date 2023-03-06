# Halo2 KZG SRS

A tool to convert KZG SRS in other formats to [`pse/halo2`](https://github.com/privacy-scaling-explorations/halo2) format.

Currently supported and WIP formats are:

- [x] [Perpetual Powers of Tau](https://github.com/weijiekoh/perpetualpowersoftau)
- [x] [SnarkJS](https://github.com/iden3/snarkjs)
- [ ] [Aztec Ignition](https://github.com/AztecProtocol/Setup)

## Download the converted SRS

Since the conversion of large SRS takes much time to run, here are the converted SRS downloadable directly. There are also simple tests ran in CI to validate these converted SRS are well formed (same ration testing with the source).

Note that `pse/halo2` has been updated to read/write in raw format by default since [PR#111](https://github.com/privacy-scaling-explorations/halo2/pull/111).

| Curve   | Source                    | K    | File in canonical format                                                                                              | File in raw format                                                                                                            |
| ------- | ------------------------- | ---- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `bn254` | `perpetual-powers-of-tau` | `1`  | [perpetual-powers-of-tau-1](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-1)   | [perpetual-powers-of-tau-raw-1](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-1)   |
| `bn254` | `perpetual-powers-of-tau` | `2`  | [perpetual-powers-of-tau-2](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-2)   | [perpetual-powers-of-tau-raw-2](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-2)   |
| `bn254` | `perpetual-powers-of-tau` | `3`  | [perpetual-powers-of-tau-3](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-3)   | [perpetual-powers-of-tau-raw-3](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-3)   |
| `bn254` | `perpetual-powers-of-tau` | `4`  | [perpetual-powers-of-tau-4](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-4)   | [perpetual-powers-of-tau-raw-4](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-4)   |
| `bn254` | `perpetual-powers-of-tau` | `5`  | [perpetual-powers-of-tau-5](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-5)   | [perpetual-powers-of-tau-raw-5](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-5)   |
| `bn254` | `perpetual-powers-of-tau` | `6`  | [perpetual-powers-of-tau-6](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-6)   | [perpetual-powers-of-tau-raw-6](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-6)   |
| `bn254` | `perpetual-powers-of-tau` | `7`  | [perpetual-powers-of-tau-7](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-7)   | [perpetual-powers-of-tau-raw-7](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-7)   |
| `bn254` | `perpetual-powers-of-tau` | `8`  | [perpetual-powers-of-tau-8](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-8)   | [perpetual-powers-of-tau-raw-8](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-8)   |
| `bn254` | `perpetual-powers-of-tau` | `9`  | [perpetual-powers-of-tau-9](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-9)   | [perpetual-powers-of-tau-raw-9](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-9)   |
| `bn254` | `perpetual-powers-of-tau` | `10` | [perpetual-powers-of-tau-10](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-10) | [perpetual-powers-of-tau-raw-10](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-10) |
| `bn254` | `perpetual-powers-of-tau` | `11` | [perpetual-powers-of-tau-11](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-11) | [perpetual-powers-of-tau-raw-11](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-11) |
| `bn254` | `perpetual-powers-of-tau` | `12` | [perpetual-powers-of-tau-12](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-12) | [perpetual-powers-of-tau-raw-12](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-12) |
| `bn254` | `perpetual-powers-of-tau` | `13` | [perpetual-powers-of-tau-13](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-13) | [perpetual-powers-of-tau-raw-13](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-13) |
| `bn254` | `perpetual-powers-of-tau` | `14` | [perpetual-powers-of-tau-14](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-14) | [perpetual-powers-of-tau-raw-14](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-14) |
| `bn254` | `perpetual-powers-of-tau` | `15` | [perpetual-powers-of-tau-15](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-15) | [perpetual-powers-of-tau-raw-15](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-15) |
| `bn254` | `perpetual-powers-of-tau` | `16` | [perpetual-powers-of-tau-16](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-16) | [perpetual-powers-of-tau-raw-16](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-16) |
| `bn254` | `perpetual-powers-of-tau` | `17` | [perpetual-powers-of-tau-17](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-17) | [perpetual-powers-of-tau-raw-17](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-17) |
| `bn254` | `perpetual-powers-of-tau` | `18` | [perpetual-powers-of-tau-18](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-18) | [perpetual-powers-of-tau-raw-18](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-18) |
| `bn254` | `perpetual-powers-of-tau` | `19` | [perpetual-powers-of-tau-19](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-19) | [perpetual-powers-of-tau-raw-19](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-19) |
| `bn254` | `perpetual-powers-of-tau` | `20` | [perpetual-powers-of-tau-20](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-20) | [perpetual-powers-of-tau-raw-20](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-20) |
| `bn254` | `perpetual-powers-of-tau` | `21` | [perpetual-powers-of-tau-21](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-21) | [perpetual-powers-of-tau-raw-21](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-21) |
| `bn254` | `perpetual-powers-of-tau` | `22` | [perpetual-powers-of-tau-22](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-22) | [perpetual-powers-of-tau-raw-22](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-22) |
| `bn254` | `perpetual-powers-of-tau` | `23` | [perpetual-powers-of-tau-23](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-23) | [perpetual-powers-of-tau-raw-23](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-23) |
| `bn254` | `perpetual-powers-of-tau` | `24` | [perpetual-powers-of-tau-24](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-24) | [perpetual-powers-of-tau-raw-24](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-24) |
| `bn254` | `perpetual-powers-of-tau` | `25` | [perpetual-powers-of-tau-25](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-25) | [perpetual-powers-of-tau-raw-25](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-25) |
| `bn254` | `perpetual-powers-of-tau` | `26` | [perpetual-powers-of-tau-26](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-26) | [perpetual-powers-of-tau-raw-26](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/perpetual-powers-of-tau-raw-26) |
| `bn254` | `hermez`                  | `1`  | [hermez-1](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-1)                                     | [hermez-raw-1](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-1)                                     |
| `bn254` | `hermez`                  | `2`  | [hermez-2](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-2)                                     | [hermez-raw-2](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-2)                                     |
| `bn254` | `hermez`                  | `3`  | [hermez-3](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-3)                                     | [hermez-raw-3](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-3)                                     |
| `bn254` | `hermez`                  | `4`  | [hermez-4](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-4)                                     | [hermez-raw-4](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-4)                                     |
| `bn254` | `hermez`                  | `5`  | [hermez-5](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-5)                                     | [hermez-raw-5](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-5)                                     |
| `bn254` | `hermez`                  | `6`  | [hermez-6](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-6)                                     | [hermez-raw-6](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-6)                                     |
| `bn254` | `hermez`                  | `7`  | [hermez-7](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-7)                                     | [hermez-raw-7](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-7)                                     |
| `bn254` | `hermez`                  | `8`  | [hermez-8](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-8)                                     | [hermez-raw-8](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-8)                                     |
| `bn254` | `hermez`                  | `9`  | [hermez-9](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-9)                                     | [hermez-raw-9](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-9)                                     |
| `bn254` | `hermez`                  | `10` | [hermez-10](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-10)                                   | [hermez-raw-10](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-10)                                   |
| `bn254` | `hermez`                  | `11` | [hermez-11](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-11)                                   | [hermez-raw-11](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-11)                                   |
| `bn254` | `hermez`                  | `12` | [hermez-12](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-12)                                   | [hermez-raw-12](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-12)                                   |
| `bn254` | `hermez`                  | `13` | [hermez-13](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-13)                                   | [hermez-raw-13](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-13)                                   |
| `bn254` | `hermez`                  | `14` | [hermez-14](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-14)                                   | [hermez-raw-14](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-14)                                   |
| `bn254` | `hermez`                  | `15` | [hermez-15](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-15)                                   | [hermez-raw-15](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-15)                                   |
| `bn254` | `hermez`                  | `16` | [hermez-16](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-16)                                   | [hermez-raw-16](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-16)                                   |
| `bn254` | `hermez`                  | `17` | [hermez-17](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-17)                                   | [hermez-raw-17](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-17)                                   |
| `bn254` | `hermez`                  | `18` | [hermez-18](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-18)                                   | [hermez-raw-18](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-18)                                   |
| `bn254` | `hermez`                  | `19` | [hermez-19](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-19)                                   | [hermez-raw-19](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-19)                                   |
| `bn254` | `hermez`                  | `20` | [hermez-20](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-20)                                   | [hermez-raw-20](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-20)                                   |
| `bn254` | `hermez`                  | `21` | [hermez-21](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-21)                                   | [hermez-raw-21](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-21)                                   |
| `bn254` | `hermez`                  | `22` | [hermez-22](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-22)                                   | [hermez-raw-22](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-22)                                   |
| `bn254` | `hermez`                  | `23` | [hermez-23](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-23)                                   | [hermez-raw-23](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-23)                                   |
| `bn254` | `hermez`                  | `24` | [hermez-24](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-24)                                   | [hermez-raw-24](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-24)                                   |
| `bn254` | `hermez`                  | `25` | [hermez-25](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-25)                                   | [hermez-raw-25](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-25)                                   |
| `bn254` | `hermez`                  | `26` | [hermez-26](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-26)                                   | [hermez-raw-26](https://trusted-setup-halo2kzg.s3.eu-central-1.amazonaws.com/hermez-raw-26)                                   |

## Manually convert from the source

### Perpetual Powers of Tau

To get SRS with `k = 10` from latest response of Perpetual Powers of Tau, we can run:

```shell
wget https://ppot.blob.core.windows.net/public/response_0071_edward
mkdir ./srs
cargo run --release --bin convert-from-perpetual-powers-of-tau response_0071_edward ./srs/perpetual-powers-of-tau-raw- 10
```

Then it will output the SRS with `1 <= k <= 10` with path `./srs/perpetual-powers-of-tau-raw-{k}`.

### SnarkJS

To get SRS with `k = 10` from Hermez's setup, we can run:

```shell
wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_10.ptau
mkdir ./srs
cargo run --release --bin convert-from-snarkjs powersOfTau28_hez_final_10.ptau ./srs/hermez-raw- 10
```

Then it will output the SRS with `1 <= k <= 10` with path `./srs/hermez-raw-{k}`.
