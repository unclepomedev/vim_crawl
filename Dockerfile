FROM rust:1.97-slim
# check rust-toolchain.toml

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

RUN useradd --create-home --uid 10001 appuser

ENV CARGO_HOME=/home/appuser/.cargo
ENV CARGO_TARGET_DIR=/home/appuser/.cargo/target

RUN mkdir -p /home/appuser/.cargo/registry /home/appuser/.cargo/target \
    && chown -R appuser:appuser /home/appuser/.cargo

WORKDIR /app
COPY . .
RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8080"]
