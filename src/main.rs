#![allow(unused_imports)]
use clap::Parser;
use color_print::cprint;
use futures_util::StreamExt;
use langchain_rust::{
    chain::{builder::ConversationalChainBuilder, Chain},
    // Embedding
    embedding::embedder_trait::Embedder,

    embedding::ollama::ollama_embedder::OllamaEmbedder,
    // ollama client
    llm::client::Ollama,
    // fmt_message, fmt_template,
    memory::SimpleMemory,
    // message_formatter,
    // prompt::HumanMessagePromptTemplate,
    prompt_args,
    // schemas::Message,
    // template_fstring,

    //Vector Store
    schemas::Document,
    vectorstore::{surrealdb::StoreBuilder, VecStoreOptions, VectorStore},
};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Archiver {
    /// Index file or directory
    #[arg(short, long)]
    index: Option<PathBuf>,

    #[arg(short, long, default_value_t = String::from("mistral"))]
    model: String,

    #[arg(short, long, default_value_t = String::from("ws://0.0.0.0:8080"))]
    surreal_url: String,
}

#[tokio::main]
async fn main() {
    let args = Archiver::parse();

    use langchain_rust::vectorstore::VecStoreOptions;
    let llm = Ollama::default().with_model(args.model); //We initialise a simple memory,by default conveational chain have this memory, but we
                                                        //initiliase it as an example, if you dont want to have memory use DummyMemory
    let memory = SimpleMemory::new();

    // Surrealdb URL to connect.
    let database_url = std::env::var("DATABASE_URL").unwrap_or(args.surreal_url);

    let surrealdb_config = surrealdb::opt::Config::new()
        .set_strict(true)
        .capabilities(surrealdb::dbs::Capabilities::all());

    // DB object that creats a new namesapce called test and defines a table called test
    let db = surrealdb::engine::any::connect((database_url, surrealdb_config))
        .await
        .unwrap();
    db.query("DEFINE NAMESPACE test;")
        .await
        .unwrap()
        .check()
        .unwrap();
    db.query("USE NAMESPACE test; DEFINE DATABASE test;")
        .await
        .unwrap()
        .check()
        .unwrap();

    // setting namesapce and table to use from Surreal
    db.use_ns("archiver").use_db("archiver").await.unwrap();

    // Embedder
    let embedder = OllamaEmbedder::default().with_model("mxbai-embed-large");

    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .db(db)
        .vector_dimensions(1536)
        .build()
        .await
        .unwrap();

    store.initialize().await.unwrap();

    // LLM
    let chain = ConversationalChainBuilder::new()
        .llm(llm)
        //IF YOU WANT TO ADD A CUSTOM PROMPT YOU CAN UN COMMENT THIS:
        //         .prompt(message_formatter![
        //             fmt_message!(Message::new_system_message("You are a helpful assistant")),
        //             fmt_template!(HumanMessagePromptTemplate::new(
        //             template_fstring!("
        // The following is a friendly conversation between a human and an AI. The AI is talkative and provides lots of specific details from its context. If the AI does not know the answer to a question, it truthfully says it does not know.
        //
        // Current conversation:
        // {history}
        // Human: {input}
        // AI:
        // ",
        //             "input","history")))
        //
        //         ])
        .memory(memory.into())
        .build()
        .expect("Error building ConversationalChain");

    // INPUT Loop
    loop {
        // Terminal interaction
        let mut question = String::new();
        stdin()
            .read_line(&mut question)
            .expect("Failed to read line");

        let input_variables = prompt_args! {
            "input" => question,
        };

        // Setup stream
        let mut stream = chain.stream(input_variables).await.unwrap();

        // Stream to Stout
        while let Some(result) = stream.next().await {
            match result {
                Ok(data) => {
                    //If you junt want to print to stdout, you can use data.to_stdout().unwrap();
                    cprint!("<green>{}</green>", data.content);
                    stdout().flush().unwrap();
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }

        println!();
    }
}
