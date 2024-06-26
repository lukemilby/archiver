![](archiver.png)

Archiver is a command-line application designed to interact with large language models (LLMs) for testing purposes.
It leverages Ollama for model integration and SurrealDB as the vector store. The application uses [Langchain-rust](https://github.com/Abraxas-365/langchain-rust) as the 
framework for interacting with the Ollama and SurrealDB.

## Prerequisites

Before setting up Archiver, ensure you have the following installed on your system:

- Rust (latest stable version)
- `just` (command runner)
- Pandoc (load text when adding to vector db)
- Ollama
- SurrealDB


## Quick Start

Ensure **docker** is running and make sure **docker compose** is setup

```bash
docker ps
```

Install Just to run the setup and Pandocs for file indexing to surrealdb

```bash
brew install pandoc just
```

Once Pandocs and Just are installed run

```bash
just docker
``` 

Add file to SurrealDB

```bash
just add <markdown file>
```
Or

```bash
cargo run -- -a <markdown file>
```

Add directory of Markdown

```bash
cargo run -- -d <directory>
```

Finally Run Archiver and use it

```bash
just run
```

to setup SurrealDB, Ollama and pull all the dependencies for Ollama

## Change Log

#### v0.3.2
- **Quick Start with Docker**: Update to Justfile and Readme to walk through setting up Archiver with docker

#### v0.3.1
- **Check for Pandoc**: Ensure pandoc exists before running

#### v0.3.0

- **Index Directory**: Add Directory Indexing for bulk Markdown file importing

#### v0.2.0

- **Prompt**: Add prompt identifier and colored response from LLM
- **Error Handling**: Better error handling with SurrealDB and Ollama
- **Chuncking**: Added document chunking when adding files to SurrealDB
- **Mode Switch**: Switched LLM chain to a Conversational Retriever from Conversational, making the LLM more efficient with responses from the VectorDB

#### v0.1.0

- **Conversational Memory**: Maintain context across interactions with the LLM.
- **Vector Stores**: Efficient storage and retrieval of high-dimensional vectors through SurrealDB.
- **Ollama Integration**: Easy setup and use of Ollama for model management.
- **Indexing Files**: Add Markdown files to SurrealDB.

## Install Archiver

1. **Install using Cargo**:
    ```bash
    cargo install archiver
    ```
    Make sure you have SurrealDB setup and Ollama before running Archiver


1. **Download and Install SurrealDB**:
    ```bash
    curl -sSf https://install.surrealdb.com | sh
    ```

2. **Run SurrealDB**:
    ```bash
    just db_dev
    ```

### Setting up SurrealDB

Once Surreal is up and running you can use https://surrealist.app/ to access the instance of Surreal. 
But before doing that a Namespace and Table need to be setup. Fastest way to get this done is to run
Archiver once. 

1. **Set Namespace and Table**:
    ```bash
    just run
    ```

2. **Add Markdown File to the SurrealDB**: 
    ```bash
    cargo run -- -a <markdownfile>
    ```
    

### Archiver

1. **Setup from Source**:
    ```bash
    git clone https://github.com/yourusername/archiver.git
    cd archiver
    just db_dev # run in a seperate terminal
    just run
    ```

## License

This project is licensed under the Apache Livense (Version 2.0). See the [LICENSE](LICENSE.txt) file for details.


