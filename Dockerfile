FROM rust:1.87-alpine AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN apk add musl-dev;

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    set -eu; \
    cargo install --path . --root /usr/local;


FROM alpine

COPY --from=builder /usr/local/bin/pipeline-runner-oidc-server /server

EXPOSE 8000

ENTRYPOINT ["/server"]
