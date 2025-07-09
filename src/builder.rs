use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use askama::Template;
use pulldown_cmark::{Parser, html};

struct Page {
    path: PathBuf,
    content: String,
    is_markdown: bool,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct PageTemplate {
    greeting: String,
    content: String,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

pub fn build_site() -> io::Result<()> {
    let dist_dir = "dist";
    let static_dir = "static";
    
    if Path::new(dist_dir).exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir_all(dist_dir)?;
    
    if Path::new(static_dir).exists() {
        copy_dir(static_dir, &Path::new(dist_dir).join("static"))?;
    }

    let mut pages = Vec::new();
    for entry in WalkDir::new("content") {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let rel_path = path.strip_prefix("content").unwrap();
            let dest_path = Path::new(dist_dir).join(rel_path).with_extension("html");
            
            let is_markdown = path.extension().map_or(false, |ext| ext == "md");
            let content = fs::read_to_string(path)?;
            
            pages.push(Page {
                path: dest_path,
                content,
                is_markdown,
            });
        }
    }

    for page in pages {
        if let Some(parent) = page.path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let html_content = if page.is_markdown {
            let parser = Parser::new(&page.content);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            html_output
        } else {
            page.content.clone()
        };
        
        let file_stem = page.path.file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        
        let template = PageTemplate {
            greeting: file_stem,
            content: html_content,
        };
        
        fs::write(&page.path, template.render().unwrap())?;
    }

    let not_found_path = Path::new(dist_dir).join("404.html");
    let not_found_template = NotFoundTemplate {};
    fs::write(not_found_path, not_found_template.render().unwrap())?;

    Ok(())
}

fn copy_dir(src: &str, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir(entry.path().to_str().unwrap(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
} 