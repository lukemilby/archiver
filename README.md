![](archiver.png)

Archiver is a command-line application designed to interact with large language models (LLMs) for testing purposes.
It leverages Ollama for model integration and SurrealDB as the vector store. The application uses [Langchain-rust](https://github.com/Abraxas-365/langchain-rust) as the 
framework for interacting with the Ollama and SurrealDB.

## Features

- **Conversational Memory**: Maintain context across interactions with the LLM.
- **Vector Stores**: Efficient storage and retrieval of high-dimensional vectors through SurrealDB.
- **Ollama Integration**: Easy setup and use of Ollama for model management.
- **Indexting Files**: Add markdown files to SurrealDB.

## Prerequisites

Before setting up Archiver, ensure you have the following installed on your system:

- Rust (latest stable version)
- Ollama
- SurrealDB
- `just` (command runner)

## Setup


### Ollama

1. **Install Ollama**:
    ```bash
    brew install ollama
    ```

### Just

1. **Install Just**:
    ```bash
    cargo install just
    ```

### SurrealDB

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
2. **Install using Cargo**:
    ```bash
    cargo install archiver
    ```

## License

This project is licensed under the Apache Livense (Version 2.0). See the [LICENSE](LICENSE.txt) file for details.


