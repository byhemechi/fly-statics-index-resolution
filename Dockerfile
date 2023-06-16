FROM rust:1-slim-bookworm as build
WORKDIR /var/app/src
RUN echo "fn main() {}" > main.rs
WORKDIR /var/app
COPY Cargo* .
RUN cargo fetch
COPY src/* src/
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /var/app/target/release/serwor_web /usr/bin/serwor_web
COPY static /app/public

CMD serwor_web