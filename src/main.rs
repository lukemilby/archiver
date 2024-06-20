#![allow(unused_imports)]
use anyhow::{Context, Result};
use clap::Parser;
use color_print::cprint;
use futures_util::StreamExt;
use langchain_rust::{
    chain::{builder::ConversationalChainBuilder, Chain},
    // schemas::Message,
    // template_fstring,
    // Document Loader
    document_loaders::{InputFormat, Loader, PandocLoader},
    // Embedding
    embedding::{embedder_trait::Embedder, ollama::ollama_embedder::OllamaEmbedder},

    llm::client::Ollama,
    // fmt_message, fmt_template,
    memory::SimpleMemory,
    // message_formatter,
    // prompt::HumanMessagePromptTemplate,
    prompt_args,
    //Vector Store
    schemas::Document,
    vectorstore::{
        surrealdb::{Store, StoreBuilder},
        VecStoreOptions, VectorStore,
    },
};
use std::fs::metadata;
use std::path::PathBuf;
use std::process;
use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

async fn file_to_doc(file_path: PathBuf) -> Result<Vec<Document>> {
    let loader = PandocLoader::from_path(InputFormat::Markdown.to_string(), file_path)
        .await
        .context("Failed to create PandocLoader")?;

    let docs = loader
        .load()
        .await
        .unwrap()
        .map(|d| d.unwrap())
        .collect::<Vec<_>>()
        .await;
    Ok(docs)
}

#[derive(Debug, Parser)]
struct Archiver {
    /// Index file, adding directory later
    #[arg(short, long)]
    archive: Option<PathBuf>,
    /// Model to run from ollama, Default: "mistral"
    #[arg(short, long, default_value_t = String::from("mistral"))]
    model: String,

    /// SurrealDB URL, Default: "ws://localhost:8888"
    #[arg(short, long, default_value_t = String::from("ws://localhost:8888"))]
    surreal_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line args
    let args = Archiver::parse();

    // use langchain_rust::vectorstore::VecStoreOptions;
    let llm = Ollama::default().with_model(args.model);

    // Conversational Memory
    let memory = SimpleMemory::new();

    // Surrealdb URL to connect.
    let database_url = std::env::var("DATABASE_URL").unwrap_or(args.surreal_url);

    let surrealdb_config = surrealdb::opt::Config::new()
        .set_strict(true)
        .capabilities(surrealdb::dbs::Capabilities::all());

    // DB object that creats a new namesapce called archiver and defines a table called archiver
    let db = surrealdb::engine::any::connect((database_url, surrealdb_config))
        .await
        .expect("Unable to conntection to surreal");

    db.query("DEFINE NAMESPACE archiver;")
        .await
        .unwrap()
        .check()
        .unwrap();
    db.query("USE NAMESPACE archiver; DEFINE DATABASE archiver;")
        .await
        .unwrap()
        .check()
        .unwrap();

    // setting namesapce and table to use from Surreal
    db.use_ns("archiver").use_db("archiver").await.unwrap();

    // Embedder
    let embedder = OllamaEmbedder::default().with_model("nomic-embed-text");

    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .db(db)
        .vector_dimensions(768)
        .build()
        .await
        .unwrap();

    store.initialize().await.unwrap();

    // Add file to vectordb
    if let Some(doc) = args.archive {
        let docs = file_to_doc(doc).await.unwrap();

        store
            .add_documents(&docs, &VecStoreOptions::default())
            .await
            .unwrap();
        process::exit(1);
    };

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
        print!(":>");
        let mut question = String::new();
        stdin()
            .read_line(&mut question)
            .expect("Failed to read line");

        // Fetch RAG results from SurrealDB
        let results = store
            .similarity_search(
                &mut question,
                2,
                &VecStoreOptions::default().with_score_threshold(0.3),
            )
            .await
            .unwrap();

        // Append content from RAG results
        results.iter().for_each(|r| {
            question.push_str(&r.page_content);
        });

        let input_variables = prompt_args! {
            "input" => question,
        };

        // Setup stream
        let mut stream = chain.stream(input_variables).await.unwrap();

        // Stream to Stdout
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
