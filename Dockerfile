### chef thing
FROM ekidd/rust-musl-builder:1.51.0 AS chef

# install cargo
USER root
RUN cargo install cargo-chef

# create work directory
WORKDIR /voting

### planner thing
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

### builder thing
FROM chef AS builder

COPY --from=planner /voting/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin voting

### runner thing
FROM alpine AS runtime

# set work directory
WORKDIR /voting

# copy over
COPY voting.toml .
COPY --from=builder /voting/target/x86_64-unknown-linux-musl/release/voting .

# run
ENTRYPOINT ["voting"]
