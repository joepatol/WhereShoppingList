# Go Version
FROM golang:1.21-rc-alpine3.17 AS builder

# Environment variables which CompileDaemon requires to run
ENV PROJECT_DIR=/app \
    GO111MODULE=on \
    CGO_ENABLED=0 \
    GOOS=linux

# Basic setup of the container
RUN mkdir /app
COPY .. /app
WORKDIR /app

RUN go build -o supermarkt-api

FROM golang:1.21-rc-alpine3.17
COPY --from=builder /app/supermarkt-api /app/supermarkt-api
WORKDIR /app

CMD ["./supermarkt-api"]