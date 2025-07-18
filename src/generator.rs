// src/generator.rs

use askama::Template; // <-- THIS WAS THE CRITICAL MISSING LINE
use std::{fs, io};
use crate::config::{read_config, HeroConfig};
use crate::models::Post;
use crate::markdown::process_markdown_files;
use fs_extra::dir::{copy, CopyOptions};
use std::vec;
use std::path::Path;

// This is the main function your cli.rs calls.
pub fn build_site() -> io::Result<()> {
    println!("Reading configuration...");
    let config = read_config("config.toml")?;
    let paths = config.build_paths();

    println!("Preparing output directories...");
    create_output_folders()?;

    println!("Copying assets...");
    config.copy_css_if_exists(&paths.css)?;
    fs::copy("templates/blog.js", "output/js/blog.js")?;

    let options = CopyOptions::new(); // Initialize copy options
    if Path::new("templates/images").exists() {
        println!("Copying images...");
        copy("templates/images", "output", &options)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }


    
    println!("Generating homepage...");
    generate_home_page(&config.hero)?;
    
    println!("Processing blog content...");
    let posts = process_markdown_files()?;
    println!("Found {} posts. Generating blog pages...", posts.len());
    generate_blog_pages(&posts, &config.hero)?;
    generate_about_page(&config.hero)?;
    
    Ok(())
}

pub fn create_output_folders() -> io::Result<()> {
    let folders = ["output", "output/about", "output/css", "output/js", "output/blog" , ];
    for folder in folders {
        fs::create_dir_all(folder)?;
    }    
    Ok(())
}    

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    hero: &'a HeroConfig,
}

fn generate_home_page(hero_config: &HeroConfig) -> io::Result<()> {
    let template = HomeTemplate { hero: hero_config };
    let html_content = template.render().unwrap();
    fs::write("output/index.html", html_content)
}

#[derive(Template)]
#[template(path = "blog_index.html")]
struct BlogIndexTemplate<'a> {
    posts: &'a Vec<Post>,
    hero: &'a HeroConfig,
}

#[derive(Template)]
#[template(path = "post_detail.html")]
struct PostDetailTemplate<'a> {
    post: &'a Post,
    all_posts: &'a Vec<Post>,
    hero: &'a HeroConfig,
}

fn generate_blog_pages(posts: &Vec<Post>, hero_config: &HeroConfig) -> io::Result<()> {
    let blog_index_template = BlogIndexTemplate { posts, hero: hero_config };
    let blog_index_html = blog_index_template.render().unwrap();
    fs::write("output/blog/index.html", blog_index_html)?;

    for post in posts {
        let post_detail_template = PostDetailTemplate {
            post,
            all_posts: posts,
            hero: hero_config,
        };
        let post_html = post_detail_template.render().unwrap();
        let file_path = format!("output/blog/{}.html", post.slug);
        fs::write(file_path, post_html)?;
    }
    Ok(())
}


// ADD THIS NEW STRUCT AT THE END OF THE FILE
#[derive(Template)]
#[template(path = "about.html")] 
struct AboutTemplate<'a> {
    hero: &'a HeroConfig,
}

fn generate_about_page(hero_config: &HeroConfig) -> io::Result<()> {
    let template = AboutTemplate { hero: hero_config };
    let html_content = template.render().unwrap();
    fs::write("output/about/about.html", html_content)
}