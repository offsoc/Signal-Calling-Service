#
# Copyright 2022 Signal Messenger, LLC
# SPDX-License-Identifier: AGPL-3.0-only
#

ARG debian_ver=bullseye

FROM debian:${debian_ver} AS build-stage

# Update system packages.
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends --no-install-suggests curl build-essential ca-certificates protobuf-compiler \
    && update-ca-certificates

# Take in a build argument to specify RUSTFLAGS environment, usually a target-cpu.
ARG rust_flags
ENV RUSTFLAGS=$rust_flags

WORKDIR /usr/src/calling-service

COPY rust-toolchain ./

# Install Rust.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $(cat rust-toolchain)
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the source and build the project normally.
COPY . .
RUN cargo build --bin calling_frontend --release

# Create a minimal container to deploy and run the calling frontend.
FROM debian:${debian_ver}-slim AS run-stage

# Update system packages.
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends --no-install-suggests ca-certificates \
    && update-ca-certificates \
    # Install curl for ip detection.
    && apt-get install -y --no-install-recommends --no-install-suggests curl \
    # Install jq for parsing gcp metadata.
    && apt-get install -y --no-install-recommends --no-install-suggests jq \
    # Cleanup unnecessary stuff.
    && rm -rf /var/lib/apt/lists/*

COPY --from=build-stage /usr/src/calling-service/target/release/calling_frontend /usr/local/bin/
COPY frontend/docker-entrypoint.sh /usr/local/bin/

USER nobody:nogroup

# Expose http server access ports to this container.
EXPOSE 8080

ENTRYPOINT ["docker-entrypoint.sh"]
