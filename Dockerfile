FROM rust as builder

WORKDIR /usr/src/site24x7_exporter
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools
RUN cargo build --target x86_64-unknown-linux-musl --release --locked
RUN strip target/x86_64-unknown-linux-musl/release/site24x7_exporter

FROM scratch
COPY --from=builder /usr/src/site24x7_exporter/target/x86_64-unknown-linux-musl/release/site24x7_exporter /app/

ENTRYPOINT ["/app/site24x7_exporter"]
