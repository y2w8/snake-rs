build mode="":
  cargo build --{{mode}}

run:
  cargo run

check:
  cargo check

deploy:
  cargo build --release
