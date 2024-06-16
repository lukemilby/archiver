# Archiver CLI Application

![](archiver.png)

Archiver is a command-line application designed to interact with large language models (LLMs) for testing purposes. It leverages Ollama for model integration and SurrealDB for database management. The application uses Langchain-rust as the framework for interacting with the LLMs, and it includes advanced features such as conversational memory and access to vector stores.

## Features

- **Conversational Memory**: Maintain context across interactions with the LLM.
- **Vector Stores**: Efficient storage and retrieval of high-dimensional vectors.
- **SurrealDB Integration**: Seamless database operations for storing and managing data.
- **Ollama Integration**: Easy setup and use of Ollama for model management.

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
    cargo install ollama
    ```

2. **Configure Ollama**:
    - Follow the [Ollama documentation](https://ollama.dev/docs) to set up your API keys and configuration files.

### SurrealDB

1. **Download and Install SurrealDB**:
    ```bash
    cargo install surrealdb
    ```

2. **Run SurrealDB**:
    ```bash
    surreal start --log debug --user root --pass root
    ```

3. **Connect to SurrealDB**:
    - Configure the connection parameters (host, port, username, password) as per your setup.

### Archiver

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/yourusername/archiver.git
    cd archiver
    ```

2. **Build the Application**:
    ```bash
    cargo build --release
    ```

3. **Run the Application**:
    ```bash
    target/release/archiver
    ```

### `just` Commands

A `justfile` is provided to streamline common tasks. Install `just` if you haven't already:

```bash
cargo install just
```

#### Available Commands

- **Build the Application**:
    ```bash
    just build
    ```

- **Run the Application**:
    ```bash
    just run
    ```

- **Test the Application**:
    ```bash
    just test
    ```

- **Format the Code**:
    ```bash
    just fmt
    ```

- **Clean the Build**:
    ```bash
    just clean
    ```

## Configuration

Create a `.env` file in the root directory to store your configuration settings:

```ini
OLLAMA_API_KEY=your_ollama_api_key
SURREALDB_HOST=localhost
SURREALDB_PORT=8000
SURREALDB_USER=root
SURREALDB_PASS=root
```

## Usage

After setting up and configuring Archiver, you can start interacting with the CLI:

```bash
archiver --help
```

### Example Commands

- **Interact with the LLM**:
    ```bash
    archiver chat "Hello, how are you?"
    ```

- **Store Data in SurrealDB**:
    ```bash
    archiver store "Some data to store"
    ```

- **Retrieve Data from SurrealDB**:
    ```bash
    archiver retrieve "query_to_run"
    ```

## Contributing

Contributions are welcome! Please read the [contributing guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


