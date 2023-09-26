_default:
  just --list

build:
  cargo build

clean:
  cargo clean

test:
  cargo test -- --nocapture