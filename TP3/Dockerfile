#docker build -t single_stage -f single_stage.dockerfile .
#docker run -d -p 8080:8080 single_stage

FROM rust:slim-buster

WORKDIR /app

# Ajout de l'utilisateur www
RUN adduser --no-create-home --disabled-login --group --system www
RUN chown www -R /app

RUN cargo new --bin dev_ops
WORKDIR /app/dev_ops
RUN chown www -R /app/dev_ops
USER www
COPY Cargo.* ./

RUN cargo build --release
RUN rm ./src/*.rs

COPY ./src ./src
RUN rm ./target/release/deps/dev_ops*
RUN cargo build --release

CMD ["./target/release/dev_ops"]