# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.59 as build

ENV PKG_CONFIG_ALLOW_CROSS=1

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

RUN apt update && apt install lld clang -y

# Install production dependencies and build a release artifact.
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11

COPY --from=build /usr/local/cargo/bin/color-mixer-service /usr/local/bin/color-mixer-service

# Run the web service on container startup.
CMD ["color-mixer-service"]