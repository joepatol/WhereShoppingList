# Go Version
FROM golang:1.21-rc-alpine3.17

# Environment variables which CompileDaemon requires to run
ENV PROJECT_DIR=/app \
    GO111MODULE=on \
    CGO_ENABLED=0

# Basic setup of the container
RUN mkdir /app
COPY .. /app
WORKDIR /app

# Get CompileDaemon
RUN go get github.com/githubnemo/CompileDaemon
RUN go install github.com/githubnemo/CompileDaemon

ENV GOOS "linux"

# The build flag sets how to build after a change has been detected in the source code
# The command flag sets how to run the app after it has been built
CMD ["./supermarkt-api"]