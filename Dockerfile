FROM rust:1.77.2-alpine AS builder
WORKDIR /app

# Install system dependencies
RUN apk update && apk add --no-cache curl build-base

# Install Tailwind isolated CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod +x tailwindcss-linux-x64 && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

# Install WASM dependencies
RUN rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli && cargo install --locked trunk

COPY ./Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN touch src/main.rs

RUN trunk build --release
RUN tailwindcss -i ./src/tailwind.css -o ./dist/tailwind.css --minify

FROM nginx:alpine as release
COPY --from=builder /app/dist/ /usr/share/nginx/html
