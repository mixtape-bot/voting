### chef thing
FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 AS chef

# create work directory
WORKDIR /api

### planner thing
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

### builder thing
FROM chef AS builder

COPY --from=planner /api/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin api

### runner thing
FROM debian:bullseye-slim AS runtime

# set work directory
WORKDIR /api

# copy over
COPY api.toml .
COPY --from=builder /api/target/release/api /usr/local/bin

# run
ENTRYPOINT ["/usr/local/bin/api"]
