VERSION 0.8

ARG --global GIT_REVISION=main
ARG --global GIT_REPOSITORY="https://github.com/s-ted/pfx-to-pem.git"

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
    COPY --keep-ts deny.toml ./
    COPY --keep-ts --dir src ./

check-dependencies:
  FROM +source

  DO rust+CARGO --args="deny --all-features check bans license sources advisories"

cross-build:
    FROM +source

    ARG --required TARGET

    DO rust+CROSS --target $TARGET
    DO rust+COPY_OUTPUT --output=".*"

    RUN find /app/
    SAVE ARTIFACT /app/target/$TARGET/release/ /release

build-all:
    LOCALLY

    RUN mkdir -p /tmp/dist
    COPY (+cross-build/release/pfx-to-pem.exe --TARGET=x86_64-pc-windows-gnu) /tmp/dist/
    COPY (+cross-build/release/pfx-to-pem --TARGET=x86_64-unknown-linux-musl) /tmp/dist/
