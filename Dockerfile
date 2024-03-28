FROM rust:latest AS build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/mandelrust /usr/bin/

CMD [ "/usr/bin/mandelrust" ]