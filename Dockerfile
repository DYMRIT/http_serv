FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y musl-tools musl-dev \
    && apt-get install -y pkg-config libssl-dev

#RUN cargo build --release
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM debian:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/practic_project /app/practic_project

RUN apt update && apt install -y nano lighttpd

RUN chmod 755 /app/practic_project

RUN mv /etc/lighttpd/conf-available/10-cgi.conf /etc/lighttpd/conf-enabled/

CMD ["lighttpd", "-D", "-f", "/etc/lighttpd/lighttpd.conf"]