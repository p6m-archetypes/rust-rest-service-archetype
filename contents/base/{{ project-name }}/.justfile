default:
    just --list

run:
    cargo run -p {{ prefix_name }}_{{ suffix_name }}_bin

install:
    cargo install --path crates/{{ prefix_name }}_{{ suffix_name }}_bin

build:
    cargo build

check:
    cargo check

fmt:
    cargo fmt

clippy:
    cargo clippy -- -D warnings
{% if persistence ~= 'None' %}
migrate-up:
    cargo run -p {{ prefix_name }}_{{ suffix_name }}_bin -- migrate up

migrate-down:
    cargo run -p {{ prefix_name }}_{{ suffix_name }}_bin -- migrate down
{% endif %}
