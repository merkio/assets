
FROM rust:1.59-bullseye AS builder

WORKDIR /usr/src/pmanager
COPY . .

RUN apt-get update -y && apt-get install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross apt-utils build-essential && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM rust:1.59-alpine

COPY --from=builder /usr/src/pmanager/target/release/personal-manager /usr/local/bin/pmanager
CMD ["pmanager"]
