# Stage 1: Build static binary using musl
FROM rust:1.86 AS builder

# Install musl and build dependencies
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

# Copy actual source and build again
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Minimal runtime image
FROM alpine:latest AS runner

# Install certs (for HTTPS support, e.g. with reqwest/hyper)
RUN apk add --no-cache ca-certificates

# Add non-root user
RUN adduser -D -u 1000 appuser

# Copy static binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/deployrelay /usr/local/bin/deployrelay

# Set ownership and permissions
RUN chown appuser:appuser /usr/local/bin/deployrelay
USER appuser

# Expose the webhook server port
EXPOSE 3000

# Run the server
CMD ["deployrelay"]