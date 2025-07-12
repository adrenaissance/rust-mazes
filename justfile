default: run

build:
  cargo build

update:
  cargo update

run:
  cargo run

check:
  cargo check

fmt:
  cargo fmt

test:
  cargo nextest run

install-tools:
  cargo install cargo-nextest

clippy:
  cargo clippy
