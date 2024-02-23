FROM lukemathwalker/cargo-chef:latest-rust-1.76-alpine AS chef
LABEL authors="badri"
WORKDIR /app
RUN apk update && apk add lld clang

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin pokemon-graphql

FROM alpine:3 AS runtime
WORKDIR /app
RUN apk add openssl ca-certificates
COPY --from=builder /app/target/release/pokemon-graphql pokemon-graphql
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./pokemon-graphql"]
