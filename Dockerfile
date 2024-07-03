# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.76.0
ARG DNOTE_VERSION=0.15.1

FROM rust:${RUST_VERSION}-slim-bullseye AS build

WORKDIR /app

RUN apt-get update && apt-get install -y build-essential curl tar

# Download and install dnote CLI using parameterized version
ARG DNOTE_VERSION
ENV DNOTE_URL=https://github.com/dnote/dnote/releases/download/cli-v${DNOTE_VERSION}/dnote_${DNOTE_VERSION}_linux_amd64.tar.gz

RUN curl -L $DNOTE_URL -o dnote.tar.gz \
  && tar -xzvf dnote.tar.gz -C /usr/local/bin \
  && rm dnote.tar.gz

# Build the Rust project
RUN --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=.config,target=.config \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=bind,source=build.rs,target=build.rs \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  <<EOF
set -e
cargo build --locked --release
cp ./target/release/dnote-tui /bin/dnote-tui
strip /bin/dnote-tui
chmod +x /bin/dnote-tui
EOF

FROM debian:bullseye-slim AS final

# Install deps
RUN apt-get update && apt-get install -y jq vim-tiny nano 

# Set the default editor for dnote
# ENV EDITOR=nano
ENV EDITOR=vim.tiny

# Copy dnote binary and Rust project binary
COPY --from=build /usr/local/bin/dnote /usr/local/bin/dnote
COPY --from=build /bin/dnote-tui /bin/

# Copy and run the script to seed dnote db with books and notes
COPY scripts/seed-dnote-db.sh /usr/local/bin/seed-dnote-db.sh
RUN chmod +x /usr/local/bin/seed-dnote-db.sh && /usr/local/bin/seed-dnote-db.sh --no-confirm

ENTRYPOINT ["/bin/dnote-tui"]
