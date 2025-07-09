// use clap::Parser;
use crate::toml_config::Config;
use std::{fs, io, path::Path};
use std::process;
use comrak::{ComrakOptions, markdown_to_html};

mod server;
mod toml_config;

// CLI argument Parser 
//TODO : ADD CLI ARGUMENTS HERE


#[tokio::main]
async fn main() -> io::Result<()> {
    //load site configuration from config.toml
    let config: Config = toml_config::read_config("config.toml").expect("Couldn't read config.toml");

    // build path based on the config
    let css_path = format!("templates/styles-{}.css" , config.theme);
    let content_template_path = format!("templates/{}.html" , config.template);
    let home_template_path = format!("templates/{}.html" , config.home_template);

    // create necessary folders needed
    create_output_folders()?;

    // copy selected css file to output/css/style.css
    if Path::new(&css_path).exists() {
            fs::copy(&css_path, "output/css/style.css").map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to copy CSS: {}", e))
            })?;
    }

    // load main template and apply the highlighted theme
    let mut content_template = fs::read_to_string(&content_template_path).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to read post template: {}", e))
    })?;

    content_template = content_template.replace("{{highlight_theme}}", &config.code_theme);

    // store converted post (filename , slug , preview) for later homepage generation
    let mut contents = Vec::new();

    // check for contents directory 
    let content_directory = Path::new("content");

    if !content_directory.exists() {
       eprintln!("❌ 'Contents/' directory is missing. Please create it and add .md files.");
        process::exit(1);
    }
    else {
        println!("📂 Found 'Contents/' folder. Starting to generate posts...");
    }

    // read each markdown file and  convert it to HTML 
    for entry in fs::read_dir(content_directory)?{
        let entry = entry?;
        let path = entry.path();

        let filename_slug = path
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other , "Invalid Filename"))?
        .to_string_lossy()
        .to_string();

        let (content_html , preview) = match path.extension().and_then(|ext| ext.to_str()){
            Some("md") => markdown_2_html(
                path.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid path to .md file"))?,
                &content_template,
            )?,
            _ => continue,
        };

        // write full content to output directory
        fs::write(format!("output/content/{}.html", filename_slug), content_html.as_bytes()).map_err(
                |e| {
                    io::Error::new(io::ErrorKind::Other, format!("Failed to write post: {}", e))
                },
            )?;

        contents.push((filename_slug.clone(), filename_slug, preview));
    }
    
    // Generate Home page with all the posts
    let home_page_html = generate_home_page(&contents, &home_template_path)?;

    fs::write("output/index.html", home_page_html.as_bytes()).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to write index.html: {}", e))
    })?;

    println!("✅ Blog built successfully!");
    println!("🌐 Starting local server at http://localhost:3000");

    server::serve_web("output", 3000).await?;
    Ok(())
}




fn create_output_folders() -> io::Result<()> {
    let folders = [
        "output",
        "output/about",
        "output/content", 
        "output/css",
    ];

    for folder in &folders {
        let folder_path = Path::new(folder);
        if !folder_path.exists() {
            fs::create_dir(folder_path)?;
        }
    }

    Ok(())


}


//markdown to html convert function
fn markdown_2_html(markdown_path: &str, html_template: &str) -> io::Result<(String, String)> {
    let markdown_content = fs::read_to_string(markdown_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read markdown file: {}", e),
        )
    })?;

    let preview_text = extract_preview(&markdown_content, 200);

    let mut comrak_options = ComrakOptions::default();
    comrak_options.extension.table = true;
    comrak_options.extension.strikethrough = true;
    comrak_options.extension.autolink = true;
    comrak_options.extension.tasklist = true;
    comrak_options.extension.footnotes = true;
    comrak_options.parse.smart = true;
    comrak_options.render.unsafe_ = true;
    comrak_options.extension.header_ids = Some("".to_string());

    let html_body = markdown_to_html(&markdown_content, &comrak_options);
    let full_html_output = html_template.replace("{body}", &html_body);

    Ok((full_html_output, preview_text))
}




fn extract_preview(content: &str, max_length: usize) -> String {
    let clean_content = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<&str>>()
        .join(" ");
    let preview = clean_content.chars().take(max_length).collect::<String>();
    if clean_content.len() > max_length {
        format!("{}...", preview)
    } else {
        preview
    }
}



fn generate_home_page(
    posts: &[(String, String, String)],//title path and preview
    html_file: &str,
) -> io::Result<String> {
    let mut content_html = String::new();

    for (title, path, preview) in posts {
        content_html.push_str(&format!(
            r#"<article class="blog-post">
                <h2><a href="/content/{}.html">{}</a></h2>
                <p class="post-preview">{}</p>
                <a href="/content/{}.html" class="read-more">Read more →</a>
            </article>"#,
            path, title, preview, path
        ));
    }

    let template = fs::read_to_string(html_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read template: {}", e),
        )
    })?;
    let home_html = template.replace(
        "{body}",
        &format!(
            r#"<h1>Blog Posts</h1>
        <div class="blog-posts">
            {}
        </div>"#,
            content_html
        ),
    );

    Ok(home_html)
}