
run:
  RUST_LOG=debug cargo run

build:
  cargo build

db_dev:
  surreal start --log trace --bind 0.0.0.0:8080 file:archiver.db
