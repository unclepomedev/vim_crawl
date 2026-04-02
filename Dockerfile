FROM rust:1.94-slim

RUN apt-get update && apt-get install -y \
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

WORKDIR /app
COPY . .

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0"]
