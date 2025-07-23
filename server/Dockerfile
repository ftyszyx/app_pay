# Stage 1: Build
FROM rust:1.87-slim as builder
# Install build dependencies, including postgresql client and nodejs/npm for swagger-ui
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev nodejs npm curl
WORKDIR /usr/src/app
RUN mkdir -p .cargo && \
    printf '[source.crates-io]\nreplace-with = "aliyun"\n\n[source.aliyun]\nregistry = "sparse+https://mirrors.aliyun.com/crates.io-index/"\n' > .cargo/config.toml
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN mkdir -p migration/src && echo "pub fn lib() {}" > migration/src/lib.rs
RUN cargo build --release --workspace
COPY src ./src
COPY migration/src ./migration/src
RUN cargo build --release --workspace


FROM debian:12-slim as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/app_server .
EXPOSE 3000
# CMD ["./app_server"] 
CMD ["/bin/bash"]