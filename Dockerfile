# --- Build stage ---
FROM rust:1.89 AS builder

WORKDIR /app
COPY . .

# Build the release binary
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime

# Install minimal runtime deps (SSL, Postgres drivers)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    pkg-config \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Add a non-root user
RUN useradd -ms /bin/bash appuser

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/oktosync-server /usr/local/bin/oktosync-server

COPY config ./config
COPY migrations ./migrations

USER appuser

EXPOSE 3000

CMD ["/usr/local/bin/oktosync-server"]
