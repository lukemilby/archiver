
# runs archiver with cargo with logging set to debug
run:
  RUST_LOG=debug cargo run


# builds a release version of archiver
build:
  cargo build --release

# list process and checks what process is running on port 8888
where:
  ps | grep surreal
  -lsof -i tcp:8888 

# setup surreal server
db_dev:
  surreal start --log trace --bind 127.0.0.1:8888 file:archiver.db

# adds file the surrealdb
add FILE:
  -cargo run -- -a {{FILE}}

# removes surreal database file
clean:
    rm -rf archiver.db
