# Multi-stage Dockerfile for toon-mcp
# Produces a small (~50MB) production image

# Stage 1: Build
FROM rust:slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src to build dependencies first
RUN mkdir src && \
    echo 'fn main() { println!("dummy"); }' > src/main.rs && \
    echo 'pub mod core { pub mod types {} }' > src/lib.rs

# Build dependencies only (with all features for HTTP mode)
RUN cargo build --release --features full && \
    rm -rf src target/release/toon-mcp*

# Copy actual source code
COPY src ./src

# Build the real application with HTTP feature
RUN cargo build --release --features http

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash toon

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/toon-mcp /app/toon-mcp

# Set ownership
RUN chown -R toon:toon /app

# Switch to non-root user
USER toon

# Default to HTTP mode on port 8080
ENV TOON_MODE=http
ENV TOON_HOST=0.0.0.0
ENV TOON_PORT=8080

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["/app/toon-mcp"]
