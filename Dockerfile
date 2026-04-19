FROM rust:1.95-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libx11-dev \
    libasound2-dev \
    libudev-dev \
    libwayland-dev \
    libxkbcommon-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

RUN useradd --create-home --uid 10001 appuser

ENV CARGO_HOME=/home/appuser/.cargo

RUN mkdir -p /home/appuser/.cargo/registry && chown -R appuser:appuser /home/appuser/.cargo

WORKDIR /app
COPY . .
RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8080"]
