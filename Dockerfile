FROM rust:1-slim-bookworm AS chef
RUN cargo install cargo-chef --locked && \
    cargo install cargo-about --features cli --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release && \
    cargo about generate -c .config/about.toml .config/about.hbs -o THIRD_PARTY_NOTICES.html

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
COPY --from=builder /app/target/release/gramps-web-mcp-rs /usr/local/bin/gramps-web-mcp-rs
COPY --from=builder /app/THIRD_PARTY_NOTICES.html /THIRD_PARTY_NOTICES.html
ENTRYPOINT ["/usr/local/bin/gramps-web-mcp-rs"]
