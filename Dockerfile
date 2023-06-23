FROM rust:1.67 AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim AS app

COPY --from=build /app/target/release/rusty_dick /

CMD ["./rusty_dick"]