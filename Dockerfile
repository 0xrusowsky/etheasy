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

# Pre-build dependencies with dummy main.rs
COPY ./Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Build the actual project
COPY . .
RUN touch src/main.rs
RUN trunk build --release
RUN tailwindcss -i ./src/tailwind.css -o ./dist/tailwind.css --minify

# Build the final nginx image with the compiled assets
FROM nginx:alpine as release
COPY ./nginx.conf /etc/nginx/nginx.conf
# Use cache busting to ensure latest assets are always loaded
ARG CACHEBUST=$(date +%s)
COPY --from=builder /app/dist/ /usr/share/nginx/html/${CACHEBUST}
WORKDIR /usr/share/nginx/html
RUN ln -s ${CACHEBUST} latest
RUN mv latest/* . && rm -rf latest
RUN pwd && ls -al
