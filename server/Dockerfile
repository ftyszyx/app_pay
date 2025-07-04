# Stage 1: Build
FROM rust:1.87-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /usr/src/app

# Copy the manifests and lock file for the entire workspace
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/

# Create dummy source files to cache dependencies efficiently
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN mkdir -p migration/src && echo "pub fn lib() {}" > migration/src/lib.rs

# Build only the dependencies to leverage Docker cache
RUN cargo build --release --workspace

# Now, copy the actual source code
COPY src ./src
COPY migration/src ./migration/src

# Build the application with the actual source code
# This step will be much faster because dependencies are already built
RUN cargo build --release --workspace

# Stage 2: Final Image
FROM debian:12-slim

WORKDIR /usr/src/app

# Copy the built binary from the correct path in the builder stage
COPY --from=builder /usr/src/app/target/release/backend .

EXPOSE 3000

CMD ["./backend"] 