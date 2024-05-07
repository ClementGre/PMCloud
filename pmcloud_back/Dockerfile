# Stage 1 - build
FROM rust:1.78 AS builder
WORKDIR /usr/src/app

COPY . .
RUN cargo install --path .

# Stage 2 - production
FROM debian:bookworm-slim AS final
WORKDIR /usr/src/app

# Libmysqlclient-dev is required for diesel
RUN apt-get update && apt-get install -y default-libmysqlclient-dev && rm -rf /var/lib/apt/lists/*
# Compiled binary
COPY --from=builder /usr/local/cargo/bin/pmcloud_back /usr/local/bin/pmcloud_back
# Static assets
COPY --from=builder /usr/src/app/static ./static

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80
EXPOSE 80

CMD ["pmcloud_back"]
