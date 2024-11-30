# Stage 1: Insall cargo-chef & sccache
FROM rust:1.81 AS base
RUN cargo install cargo-chef
RUN cargo install sccache
ENV RUSTC_WARPER=sccache SCCACHE_DIR=/sccache

# Stage 2: Cache Dependencies wih cargo-chef & sccache
FROM base AS planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
  cargo chef prepare --recipe-path recipe.json

# Stage 3: Build the Image
FROM base AS builder
ARG DATABASE_URL
ENV DATABASE_URL ${DATABASE_URL}
ENV TEST_DATABASE_URL ${DATABASE_URL}

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
  cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
  cargo build --release

# Stage 4: reate a slim runtime image
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/rust-auth-service /usr/local/bin/rust-auth-service
CMD ["rust-auth-service"]
