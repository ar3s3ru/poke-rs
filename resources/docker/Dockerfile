# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

# Targeting MUSL for lightweight standard library
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/poke-rs

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 poke
RUN adduser -D -s /bin/sh -u 1000 -G poke poke

WORKDIR /home/poke-rs/bin/

COPY --from=cargo-build /usr/src/poke-rs/target/x86_64-unknown-linux-musl/release/poke .
RUN chown poke:poke poke

USER poke

CMD ["./poke", "web"]
