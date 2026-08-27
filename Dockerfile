FROM rust:1.98-bookworm AS build
WORKDIR /build
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --locked --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates git && rm -rf /var/lib/apt/lists/*
COPY --from=build /build/target/release/forge-sync /usr/local/bin/forge-sync
WORKDIR /data
ENTRYPOINT ["forge-sync"]
CMD ["--help"]
