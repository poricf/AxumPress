# Learning Rust: Part 3 - Building a Static Site Generator with Askama

In this post, we'll walk through how this very blog engine was built using Rust, Markdown, and the [Askama](https://github.com/djc/askama) templating engine.

## Why Rust and Askama?

Rust offers safety, speed, and a great developer experience. Askama brings Jinja2-style templates to Rust, making it easy to generate HTML from Rust structs.

## Project Structure

```text
axumpress/
├── content/           # Markdown blog posts
├── output/            # Generated static site
├── src/               # Rust source code
├── templates/         # Askama HTML templates
├── Cargo.toml         # Rust dependencies
└── config.toml        # Site config
```

## Example: Rendering a Blog Post

Here's how a blog post is rendered using Askama:

```rust
#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub body: &'a str, // HTML
}

// In main.rs
let post_template = PostTemplate {
    title: "Learning Rust: Part 3",
    body: &html_body,
};
let rendered = post_template.render()?;
```

## Features

- Markdown to HTML conversion with [Comrak](https://github.com/kivikakk/comrak)
- Fast static site generation
- Modern, customizable CSS themes
- Easy deployment to static hosts (Cloudflare Pages, Netlify, etc.)

## About This Project

This project is open source and built for learning and sharing. You can find the code on [GitHub](https://github.com/poricf/axumpress) and adapt it for your own blog or documentation site.

---

Happy blogging with Rust and Askama!
