
# runs archiver with cargo with logging set to debug
run:
  RUST_LOG=debug cargo run


# builds a release version of archiver
build:
  cargo build --release

# deploy archiver stack
docker:
  docker compose up -d
  docker exec -it ollama ollama pull mistral
  docker exec -it ollama ollama pull nomic-embed-text


# install pandocs
install_deps: 
  brew install pandoc just ollama
  curl -fsSL https://get.docker.com -o get-docker.sh
  sh get-docker.sh
  curl -sSf https://install.surrealdb.com | sh
  ollama pull mistral
  ollama pull nomic-embed-text


# list process and checks what process is running on port 8888
where:
  ps | grep surreal
  -lsof -i tcp:8888 

# setup surreal server
setup_local:
  surreal start --log trace --bind 127.0.0.1:8888 file:archiver.db

# adds file the surrealdb
add FILE:
  -cargo run -- -a {{FILE}}

# removes surreal database file
clean:
    rm -rf archiver.db
