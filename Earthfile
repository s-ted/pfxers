VERSION 0.8

IMPORT github.com/earthly/lib/rust:3.0.2 AS rust
FROM alpine:3.19

install:
    FROM rust:1.80-slim-bookworm
    DO github.com/earthly/lib+INSTALL_DIND

    RUN cargo install --locked cargo-deny
    DO rust+INIT --keep_fingerprints=true

source:
    FROM +install
    WORKDIR /app

    COPY --keep-ts Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir src ./

cross-build:
    FROM +source

    ARG --required TARGET

    DO rust+CROSS --target $TARGET
    DO rust+COPY_OUTPUT --output=".*"

    SAVE ARTIFACT /app/target/$TARGET/release/ /release

build-all:
    LOCALLY

    LET RELEASE_VERSION="$(cargo semver get)"

    COPY (+cross-build/release/pfxers --TARGET=x86_64-unknown-linux-musl) "target/pfxers-${RELEASE_VERSION}-x86_64-unknown-linux-musl/"
    RUN cp LICENCE-APACHE LICENCE-MIT "target/pfxers-${RELEASE_VERSION}-x86_64-unknown-linux-musl/"
    RUN tar -I 'gzip -9' -cvf "target/pfxers-${RELEASE_VERSION}-x86_64-unknown-linux-musl.tar.gz" -C target "pfxers-${RELEASE_VERSION}-x86_64-unknown-linux-musl/"

    COPY (+cross-build/release/pfxers.exe --TARGET=x86_64-pc-windows-gnu) "target/pfxers-${RELEASE_VERSION}-x86_64-pc-windows-gnu/"
    RUN cp LICENCE-APACHE LICENCE-MIT "target/pfxers-${RELEASE_VERSION}-x86_64-pc-windows-gnu/"
    RUN (cd target && zip -r9 "pfxers-${RELEASE_VERSION}-x86_64-pc-windows-gnu.zip" "pfxers-${RELEASE_VERSION}-x86_64-pc-windows-gnu/")
