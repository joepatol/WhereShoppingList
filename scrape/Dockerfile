FROM rust:1.75.0-alpine as builder

RUN apk update
RUN apk upgrade
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
COPY . /func
WORKDIR /func/scraper
ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN cargo build --release

FROM rust:1.75.0-alpine
WORKDIR /opt/func
COPY --from=builder /func/scraper/target/release/scraper .

CMD ["./scraper"]