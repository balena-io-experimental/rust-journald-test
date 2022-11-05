FROM ubuntu:20.04 as build

RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates gcc pkg-config libsystemd-dev libc6-dev llvm && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN . "$HOME/.cargo/env" && \
    cargo build --profile release && \
    llvm-strip target/release/log-haul

FROM ubuntu:20.04

COPY --from=build /usr/src/app/target/release/log-haul /

CMD ["/log-haul"]
