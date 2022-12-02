FROM rust:1.64.0

RUN apt-get update && apt-get install -y curl libpq-dev build-essential

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo install diesel_cli --no-default-features --features postgres && cargo build --release

RUN echo "DATABASE_URL=postgres://username:password@database_ip_address/game_reviews" >> .env && diesel setup

COPY ./target/release/game_rocket \
  /usr/local/bin/

WORKDIR /root
CMD ROCKET_PORT=$PORT /usr/local/bin/game_rocket
