mod config;
mod generator;
mod markdown;
mod server;

use crate::config::{Config, read_config};
use crate::generator::{create_output_folders, generate_home_page};
use crate::markdown::process_markdown_files;

use std::{fs, io};
use tokio;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Load config
    let config = read_config("config.toml")?;
    let paths = config.build_paths();

    // Prepare folders
    create_output_folders()?;

    // Copy CSS
    config.copy_css_if_exists(&paths.css)?;

    // Load template and inject highlight theme
    let mut content_template = config.load_template(&paths.content_template)?;
    content_template = content_template.replace("{{highlight_theme}}", &config.code_theme);

    // Process markdown posts
    let contents = process_markdown_files("content", &content_template)?;

    // Generate homepage
    let home_html = generate_home_page(&contents, &paths.home_template)?;
    fs::write("output/index.html", home_html)?;

    // Copy about page
    config.copy_about_page_if_exists()?;

    println!("✅ Blog built successfully!");
    println!("🌐 Starting local server at http://localhost:3000");

    server::serve_web("output", 3000).await
}
