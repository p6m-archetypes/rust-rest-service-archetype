# syntax=docker/dockerfile:1
FROM rust:latest AS builder
WORKDIR /app
COPY . .
# BuildKit cache mounts keep the cargo registry and build artifacts across image builds,
# so only changed crates recompile. The binary is copied out because cache mounts are
# not part of the image filesystem.
RUN --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo build --release -p {{ project_name }}_bin \
    && install -m 0755 target/release/{{ project-name }} /usr/local/bin/{{ project-name }}

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd --system --no-create-home --shell /sbin/nologin appuser
COPY --from=builder /usr/local/bin/{{ project-name }} /usr/local/bin/
USER appuser
EXPOSE {{ service_port }}
EXPOSE {{ management_port }}
ENTRYPOINT ["{{ project-name }}"]
