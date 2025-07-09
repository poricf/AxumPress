use axum::{
    routing::get,
    Router,
    response::Html,
};
use askama::Template;
use std::net::SocketAddr;
use clap::{Parser, Subcommand};

mod builder;

#[derive(Parser)]
#[command(name = "my_blog")]
#[command(about = "Static site generator and server", version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve,
    Build,
}

#[derive(Template)]
#[template(path = "index.html")]
struct PageTemplate {
    greeting: String,
    content: String,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

async fn home() -> Html<String> {
    let template = PageTemplate {
        greeting: "Hello World!".to_string(),
        content: "Welcome to my blog".to_string(),
    };
    Html(template.render().unwrap())
}

async fn not_found() -> Html<String> {
    let template = NotFoundTemplate {};
    Html(template.render().unwrap())
}

async fn run_server() {
    let app = Router::new()
        .route("/", get(home))
        .fallback(not_found);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Serve => run_server().await,
        Commands::Build => {
            if let Err(e) = builder::build_site() {
                eprintln!("❌ Build failed: {}", e);
                std::process::exit(1);
            }
            println!("✅ Site built successfully in dist/");
        }
    }
}