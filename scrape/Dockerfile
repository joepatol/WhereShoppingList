FROM rust:1.75.0-alpine as builder

RUN apk update
RUN apk upgrade
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
COPY . /func
WORKDIR /func/scraper
ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN cargo build --release

FROM alpine:latest
WORKDIR /opt/func
COPY --from=builder /func/scraper/target/release/scraper .

EXPOSE 7071
CMD ["./scraper"]