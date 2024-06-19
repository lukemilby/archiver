
run:
  RUST_LOG=debug cargo run

build:
  cargo build

where:
  ps | grep surreal
  lsof -i tcp:8888 

db_dev:
  surreal start --log trace --bind 127.0.0.1:8888 file:archiver.db

clean:
    rm -rf archiver.db
