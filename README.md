[![Continuous integration](https://github.com/s-ted/pfxers/workflows/CI/badge.svg)](https://github.com/s-ted/pfxers/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/pfxers.svg)](https://crates.io/crates/pfxers/1.0.1)
[![HitCount](https://hits.dwyl.com/s-ted/pfxers.svg?style=flat-square&show=unique)](http://github.com/s-ted/pfxers)

# pfxers

`pfxers` allows you to look into PFX or PEM files, display their properties
and copy their contents (certificate, certificate chains, key).

### Cargo

[![Crates.io](https://img.shields.io/crates/v/pfxers)](https://crates.io/crates/pfxers/1.0.1)

```shell
cargo install pfxers --locked
```

## Usage Examples

Basic usage:

```shell
pfxers certificate.crt
```

Using a password protected PFX file:

```shell
pfxers password-protected.pfx --password 'thisissecret'
```

Using a password protected PFX file, the password being in a file:

```shell
pfxers password-protected.pfx --password-file password.txt
```

## Command Reference

```text
Usage: pfxers [OPTIONS] <INPUT>

Arguments:
  <INPUT>  The PFX/PKCS12/pem file to inspect

Options:
      --password-file <PASSWORD_FILE>
          The file containing the password of the PFX/PKCS12 file
      --password <PASSWORD>
          The  password of the PFX/PKCS12 file You should prefer the use of --password-file or
          use the PFX_PASSWORD environment variable [env: PFX_PASSWORD=]
  -h, --help
          Print help
  -V, --version
          Print version
```
## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Copyright 2024 [pfxers Contributors](https://github.com/s-ted/pfxers/graphs/contributors)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
