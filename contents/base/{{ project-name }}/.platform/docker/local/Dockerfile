FROM rust:latest AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release -p {{ prefix_name }}_{{ suffix_name }}_bin

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd --system --no-create-home --shell /sbin/nologin appuser
COPY --from=builder /app/target/release/{{ project-name }} /usr/local/bin/
USER appuser
EXPOSE {{ service_port }}
EXPOSE {{ management_port }}
ENTRYPOINT ["{{ project-name }}"]
