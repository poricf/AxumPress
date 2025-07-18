// src/markdown.rs

use crate::models::{FrontMatter, Post};
use comrak::{markdown_to_html, ComrakOptions};
use gray_matter::{engine::YAML, Matter};
use serde::Deserialize;
use std::{fs, io};

pub fn process_markdown_files() -> io::Result<Vec<Post>> {
    let content_dir = std::path::Path::new("content");
    println!("[DEBUG] Searching for markdown files in: {:?}", content_dir.canonicalize().unwrap_or_else(|_| content_dir.to_path_buf()));

    if !content_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "'content' directory not found at project root."));
    }

    let mut posts = Vec::new();
    let matter = Matter::<YAML>::new();

    for entry in fs::read_dir(content_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            println!("[DEBUG] Processing file: {:?}", &path);

            let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            let raw_content = fs::read_to_string(&path)?;

            let parsed_content = matter.parse(&raw_content);
            if let Some(data) = parsed_content.data {
                match data.deserialize::<FrontMatter>() {
                    Ok(front_matter) => {
                        println!("[SUCCESS] Parsed front-matter for: {}", &slug);
                          let markdown_body = &parsed_content.content;

                        let word_count = markdown_body.split_whitespace().count();
                        const WORDS_PER_MINUTE: f64 = 225.0;
                        let reading_time_minutes = (word_count as f64 / WORDS_PER_MINUTE).ceil() as u32;

                        
                        let mut options = ComrakOptions::default();
                        options.render.unsafe_ = true;
                        options.extension.header_ids = Some("".to_string());
                        let html_content = markdown_to_html(&parsed_content.content, &options);

                        posts.push(Post {
                            front_matter,
                            slug,
                            html_content,
                            reading_time_minutes
                        });
                    },
                    Err(e) => {
                        println!("[ERROR] Failed to deserialize front-matter for file {:?}: {}", &path, e);
                    }
                }
            } else {
                println!("[WARN] No front-matter found for file: {:?}", &path);
            }
        }
    }

    posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));
    println!("[INFO] Total posts processed successfully: {}", posts.len());
    Ok(posts)
}