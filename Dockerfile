# Stage 1: Build static binary
FROM golang:1.24-alpine AS builder

WORKDIR /app

# Copy go mod files and download dependencies
COPY go.mod go.sum ./
RUN go mod download

# Copy source code and build
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o deployrelay .

# Stage 2: Minimal runtime image
FROM alpine:latest AS runner

WORKDIR /app

# Install certs (for HTTPS support, e.g. with reqwest/hyper)
RUN apk add --no-cache ca-certificates

# Copy static binary
COPY --from=builder /app/deployrelay .

# Expose the webhook server port
EXPOSE 3000

# Run the server
CMD ["/app/deployrelay"]