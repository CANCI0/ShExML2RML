# Dockerfile for Rust API and Vite React Webapp (multi-stage)

# --- Build Rust API ---
FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# --- Build Vite Webapp ---
FROM node:20 AS webapp-builder
WORKDIR /webapp
COPY webapp/package.json webapp/package-lock.json ./
RUN npm install --legacy-peer-deps
COPY webapp .
RUN npm run build

# --- Final image ---
FROM debian:stable-slim
WORKDIR /app
# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates npm && rm -rf /var/lib/apt/lists/*
# Copy Rust binary
COPY --from=builder /app/target/release/shexml2rml /usr/local/bin/shexml2rml
# Copy webapp static files
COPY --from=webapp-builder /webapp/dist /app/webapp
# Expose port for API (default 8080) and web (default 4173 or 80)
EXPOSE 8080 80
# Entrypoint: run API (with --api) and serve static files
CMD ["sh", "-c", "shexml2rml --api && npx serve -s /app/webapp -l 80"]
