VERSION 0.8

IMPORT github.com/earthly/lib/rust:3.0.2 AS rust
FROM alpine:3.19

install:
    FROM rust:1.77-slim-bookworm
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

    COPY (+cross-build/release/pfxers --TARGET=x86_64-unknown-linux-musl) target/
    COPY (+cross-build/release/pfxers.exe --TARGET=x86_64-pc-windows-gnu) target/

publish-crate:
    FROM +source

    RUN cargo publish
