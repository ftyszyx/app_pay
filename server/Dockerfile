# Stage 1: Build
FROM rust:1.87-slim as builder
# Install build dependencies, including postgresql client and nodejs/npm for swagger-ui
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev nodejs npm curl
WORKDIR /usr/src/app
RUN mkdir -p .cargo && \
    printf '[source.crates-io]\nreplace-with = "aliyun"\n\n[source.aliyun]\nregistry = "sparse+https://mirrors.aliyun.com/crates.io-index/"\n' > .cargo/config.toml
COPY Cargo.toml Cargo.lock ./
# Copy workspace members' manifests for dependency caching
COPY migration/Cargo.toml ./migration/
COPY entity/Cargo.toml ./entity/
COPY pay/Cargo.toml ./pay/
COPY xt-oss/Cargo.toml ./xt-oss/
COPY aliyun-sts/Cargo.toml ./aliyun-sts/
# Create minimal sources so the first workspace build succeeds
RUN mkdir -p src && echo "fn main() {}" > src/main.rs \
 && mkdir -p migration/src && echo "pub fn lib() {}" > migration/src/lib.rs \
 && mkdir -p entity/src && echo "pub fn lib() {}" > entity/src/lib.rs \
 && mkdir -p pay/src && echo "pub fn lib() {}" > pay/src/lib.rs \
 && mkdir -p xt-oss/src && echo "pub fn lib() {}" > xt-oss/src/lib.rs \
 && mkdir -p aliyun-sts/src && echo "pub fn lib() {}" > aliyun-sts/src/lib.rs
RUN cargo build --release --workspace
# Now copy real sources and build the actual binaries
COPY src ./src
COPY migration/src ./migration/src
COPY entity/src ./entity/src
COPY pay/src ./pay/src
COPY xt-oss/src ./xt-oss/src
COPY aliyun-sts/src ./aliyun-sts/src
RUN cargo build --release --workspace


FROM debian:12-slim as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/app_server .
EXPOSE 3000
CMD ["./app_server"]