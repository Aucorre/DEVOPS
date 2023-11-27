FROM rust:slim-buster AS builder

WORKDIR /build

RUN adduser --group --no-create-home --disabled-login --system builder
RUN chown -R builder /build
USER builder

RUN cargo new --bin dev_ops
WORKDIR /build/dev_ops

COPY Cargo.* ./
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/dev_ops*

COPY ./src ./src
RUN cargo build --release

FROM scratch
COPY --from=builder /build/dev_ops/target/release/dev_ops /app
CMD ["/app"]