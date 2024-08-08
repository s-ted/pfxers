[![Continuous integration](https://github.com/s-ted/pfx-to-pem/workflows/CI/badge.svg)](https://github.com/s-ted/pfx-to-pem/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/pfx-to-pem.svg)](https://crates.io/crates/pfx-to-pem/1.0.0)
[![Packaging status](https://repology.org/badge/tiny-repos/pfx-to-pem.svg)](https://repology.org/project/pfx-to-pem/versions)
[![HitCount](https://hits.dwyl.com/s-ted/pfx-to-pem.svg?style=flat-square&show=unique)](http://github.com/s-ted/pfx-to-pem)

# Pfx-to-pem

Pfx-to-pem allows you to look into PFX or PEM files, display their properties
and copy their contents (certificate, certificate chains, key).

### Cargo

[![Crates.io](https://img.shields.io/crates/v/pfx-to-pem)](https://crates.io/crates/pfx-to-pem/1.0.0)

```shell
cargo install trippy --locked
```

## Usage Examples

Basic usage:

```shell
pfx-to-pem certificate.crt
```

Using a password protected PFX file:

```shell
pfx-to-pem password-protected.pfx --password 'thisissecret'
```

Using a password protected PFX file, the password being in a file:

```shell
pfx-to-pem password-protected.pfx --password-file password.txt
```

## Command Reference

```text
Usage: pfx-to-pem [OPTIONS] <INPUT>

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

This project is distributed under the terms of the MIT license.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in time by you, as defined
in the MIT license, shall be licensed as above, without any additional terms or conditions.

See [LICENSE](LICENSE.txt) for details.

Copyright 2024 [pfx-to-pem Contributors](https://github.com/s-ted/pfx-to-pem/graphs/contributors)
