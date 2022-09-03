FROM docker.io/ubuntu
COPY --chmod=755 site24x7_exporter /app/
ENTRYPOINT ["/app/site24x7_exporter"]
