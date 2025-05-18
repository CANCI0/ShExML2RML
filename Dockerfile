# --- Build Rust API ---
FROM rust:1.85 as builder
WORKDIR /app

# Optimized cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release

# --- Build Vite Webapp ---
FROM node:20 as webapp-builder
WORKDIR /webapp
COPY webapp/package.json webapp/package-lock.json ./
RUN npm install --legacy-peer-deps
COPY webapp . 
RUN npm run build

# --- Final image ---
FROM debian:stable-slim as final
WORKDIR /app

ENV RUST_ENV=development

RUN apt-get update && apt-get install -y \
    curl \
    gnupg2 \
    ca-certificates \
    debian-keyring \
    debian-archive-keyring && \
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/caddy-stable-archive-keyring.gpg] https://dl.cloudsmith.io/public/caddy/stable/deb/debian any-version main" \
    > /etc/apt/sources.list.d/caddy-stable.list && \
    apt-get update && \
    apt-get install -y caddy && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/shexml2rml /usr/local/bin/shexml2rml
COPY --from=webapp-builder /webapp/dist /app/webapp

EXPOSE 8080 80

CMD ["sh", "-c", "shexml2rml --api & caddy file-server --root /app/webapp --listen :80"]
