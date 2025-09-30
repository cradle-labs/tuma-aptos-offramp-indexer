# Use official Rust image as builder
FROM rust:1.86-bookworm as chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Install system dependencies needed for compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libdw-dev \
    && rm -rf /var/lib/apt/lists/*

# Build dependencies - this layer will be cached unless dependencies change
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code and build the application
COPY . .
RUN cargo build --release

# Runtime stage - minimal image
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN useradd -r -s /bin/false appuser

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/tuma-indexer /app/tuma-indexer

# Copy configuration template
COPY processor.yaml.tmpl /app/

# Change ownership to app user and ensure write permissions for config creation
RUN chown -R appuser:appuser /app && chmod 755 /app

# Switch to non-root user
USER appuser

# Expose port (adjust if your app uses a different port)
EXPOSE 7979

# Run the binary
CMD ["./tuma-indexer", "--config-path", "config.yaml"]