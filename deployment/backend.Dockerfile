# syntax=docker/dockerfile:1

# Create a stage for building the application.

ARG RUST_VERSION=1.80.0
ARG APP_NAME=split
FROM rust:${RUST_VERSION}-slim-bullseye AS build
LABEL stage=builder
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y \
	build-essential \
	curl \
	openssl \
	libssl-dev \
	pkg-config

# Build the application.
RUN --mount=type=bind,source=src,target=src \
	--mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
	--mount=type=bind,source=migrations,target=migrations \
	--mount=type=bind,source=openapi-models,target=openapi-models \
	<<EOF
set -e
cargo build --release --locked
cp ./target/release/$APP_NAME /bin/server
EOF

# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM debian:11 AS final

LABEL stage=final

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

EXPOSE 8080

CMD ["/bin/server"]
