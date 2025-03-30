# Use the official Rust image to build the binary
FROM rust:1.77 AS builder

# Set the working directory
WORKDIR /src

# Ensure Cargo is up-to-date
RUN rustup update stable && rustup default stable

# Install dependencies needed for Rust crates
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy only Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Pre-build dependencies to leverage caching
RUN mkdir -p src && echo "fn main() {}" > src/lib
RUN cargo build --release || true  # Allow failure due to missing dependencies

# Copy the full source code
COPY . .  

# Build the final Rust binary
RUN cargo build --release

# Use a newer Debian base image with GLIBC 2.36+
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /src/target/release/shahi /usr/local/bin/shahi

# Ensure it's executable
RUN chmod +x /usr/local/bin/shahi

# Expose the required port
EXPOSE 8080

# Set the entry point
ENTRYPOINT ["/usr/local/bin/shahi"]
