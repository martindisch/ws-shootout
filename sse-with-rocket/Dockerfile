# Builder ---------------------------------------------------------------------
FROM rust:1.61-slim AS builder
WORKDIR /usr/src/sse-with-rocket

# Stage 1: install & compile dependencies (only runs when dependencies changed)
RUN cargo init
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

# Stage 2: copy code & build (only runs when code changed)
COPY . .
# Sleeping and touching before building is necessary so the timestamp of
# main.rs is not the same it was when we initialized the empty project for
# dependency caching
RUN sleep 1 && touch src/main.rs && cargo build --release

# Final image -----------------------------------------------------------------
FROM debian:11-slim
COPY --from=builder \
  /usr/src/sse-with-rocket/target/release/sse-with-rocket \
  /usr/local/bin
COPY --from=builder \
  /usr/src/sse-with-rocket/Rocket.toml \
  .
CMD ["sse-with-rocket"]
