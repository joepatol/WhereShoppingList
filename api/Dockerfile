# syntax=docker/dockerfile:1

FROM golang:1.21

WORKDIR /app

# Download Go modules
COPY ./src ./
RUN go mod download

RUN CGO_ENABLED=0 GOOS=linux go build -o /supermarkt-api
CMD ["/supermarkt-api"]