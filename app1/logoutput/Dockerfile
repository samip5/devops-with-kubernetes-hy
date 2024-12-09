FROM rust:1.73-alpine as builder

RUN apk add --no-cache musl-dev gcc libgcc

WORKDIR /usr/src/logoutput
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo fetch

RUN cargo build --release


FROM alpine:3.20

COPY --from=builder /usr/src/logoutput/target/release/logoutput /usr/local/bin/logoutput

CMD ["logoutput"]