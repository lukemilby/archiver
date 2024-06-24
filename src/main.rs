#![allow(unused_imports)]
use anyhow::{Context, Result};
use clap::Parser;
use color_print::cprint;
use futures_util::StreamExt;
use langchain_rust::{
    add_documents,
    chain::{builder::ConversationalChainBuilder, Chain, ConversationalRetrieverChainBuilder},
    // schemas::Message,
    // template_fstring,
    // Document Loader
    document_loaders::{InputFormat, Loader, PandocLoader},
    // Embedding
    embedding::{embedder_trait::Embedder, ollama::ollama_embedder::OllamaEmbedder},
    fmt_message,
    fmt_template,
    llm::client::Ollama,
    memory::SimpleMemory,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::{Document, Message},
    template_fstring,
    //Vector Store
    template_jinja2,
    text_splitter::{MarkdownSplitter, SplitterOptions, TextSplitter},
    vectorstore::{
        surrealdb::{Store, StoreBuilder},
        Retriever, VecStoreOptions, VectorStore,
    },
};
use std::fs::metadata;
use std::path::PathBuf;
use std::process;
use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

// Document Chunker
async fn chunk(docs: &Vec<Document>) -> Result<Vec<Document>> {
    let options = SplitterOptions::default();
    let md_splitter = MarkdownSplitter::new(options);
    let docs = md_splitter.split_documents(&docs).await?;
    println!("{:?}", docs.len());
    Ok(docs)
}

// Document Loader
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

    let chunked_docs = chunk(&docs).await?;

    Ok(chunked_docs)
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
        let documents = file_to_doc(doc).await.unwrap();

        let _ = add_documents!(store, &documents).await.map_err(|e| {
            println!("Error adding documents: {:?}", e);
        });
        process::exit(1);
    };

    // Prompt
    let prompt= message_formatter![
                    fmt_message!(Message::new_system_message("You are a helpful assistant")),
                    fmt_template!(HumanMessagePromptTemplate::new(
                    template_jinja2!("
Use the following pieces of context to answer the question at the end. If you don't know the answer, just say that you don't know, don't try to make up an answer.

{{context}}

Question:{{question}}
Helpful Answer:

        ",
                    "context","question")))

                ];

    // LLM
    let chain = ConversationalRetrieverChainBuilder::new()
        .llm(llm)
        .rephrase_question(true)
        .prompt(prompt)
        .retriever(Retriever::new(store, 5))
        .memory(memory.into())
        .build()
        .expect("Error building Retriever Chain");

    // INPUT Loop
    // Where the interactions with the LLM are all put together
    loop {
        // Terminal interaction
        print!("|[●▪▪●]|> ");
        let _ = stdout().flush();
        let mut question = String::new();
        stdin()
            .read_line(&mut question)
            .expect("Failed to read line");

        // Fetch RAG results from SurrealDB
        // COMMMENTED for retriever llm chain upgrade. Documents should be queried from the LLM Chain vs manually here
        // let results = store
        //     .similarity_search(
        //         &mut question,
        //         2,
        //         &VecStoreOptions::default().with_score_threshold(0.4),
        //     )
        //     .await
        //     .expect("Error with fetching docs with Surreal or Ollama");

        // Append content from RAG results
        // results.iter().for_each(|r| {
        //     question.push_str(&r.page_content);
        // });

        // LLM Chain
        // How would I use this?
        let input_variables = prompt_args! {
            "question" => question,
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
