FROM rust:latest
RUN apt update && apt install libdbus-1-dev pkg-config -y
RUN cargo install cargo-audit
RUN rustup component add clippy
RUN rustup component add rustfmt
