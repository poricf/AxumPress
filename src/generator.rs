use std::{fs, io, path::Path};

pub fn create_output_folders() -> io::Result<()> {
    let folders = ["output", "output/about", "output/content", "output/css"];
    for folder in folders {
        if !Path::new(folder).exists() {
            fs::create_dir(folder)?;
        }
    }
    Ok(())
}

pub fn generate_home_page(
    posts: &[(String, String, String)],
    template_path: &str,
) -> io::Result<String> {
    let mut posts_html = String::new();

    for (title, path, preview) in posts {
        posts_html.push_str(&format!(
            r#"<article class="blog-post">
                <h2><a href="/content/{}.html">{}</a></h2>
                <p class="post-preview">{}</p>
                <a href="/content/{}.html" class="read-more">Read more →</a>
            </article>"#,
            path, title, preview, path
        ));
    }

    let template = fs::read_to_string(template_path)?;
    let final_html = template.replace(
        "{body}",
        &format!(
            r#"<h1>Blog Posts</h1>
               <div class="blog-posts">{}</div>"#,
            posts_html
        ),
    );

    Ok(final_html)
}
