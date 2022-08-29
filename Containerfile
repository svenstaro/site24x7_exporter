FROM docker.io/rust as builder

WORKDIR /usr/src/site24x7_exporter
COPY . .
RUN cargo build --release --locked

FROM docker.io/ubuntu
COPY --from=builder /usr/src/site24x7_exporter/target/release/site24x7_exporter /app/

ENTRYPOINT ["/app/site24x7_exporter"]
