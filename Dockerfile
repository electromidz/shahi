# Use the official Rust image as a base image
#FROM rust:latest as builder
FROM rust:1.77 AS builder


# Set the working directory
WORKDIR /src

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to pre-compile dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies (this step will cache the dependencies)
RUN cargo build --release

# Remove the dummy main.rs file
RUN rm -rf src

# Copy the rest of the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:buster-slim

# Install necessary libraries (if any)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/your_app_name /usr/local/bin/your_app_name

# Set the entry point to run the application
ENTRYPOINT ["/usr/local/bin/shahi"]
