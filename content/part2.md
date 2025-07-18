---
title: "How I Built This Blog (Part 2): The User Experience with Clap and Askama"
date: "2025-07-28"
category: "Projects"
tags: ["rust", "cli", "askama", "clap"]
excerpt: "A static site generator is useless without a good user experience. In Part 2, I dive into how I built an interactive CLI with `clap` and used the powerful `askama` templating engine to finally bring the data from Part 1 to the screen."
# image: "/images/placeholder-part2.jpg"
---

In Part 1, we built the core data pipeline for AxumPress. We successfully turned a folder of markdown files into a structured `Vec<Post>` in our Rust program's memory. This was a huge step, but data in memory isn't a website. A website is what the user *sees*.

I realized that if this tool was going to be useful to anyone else (or even to my future self), it couldn't just be a script I ran. It needed a proper user experience. It needed to be friendly, interactive, and powerful.

This led to a new goal: **since the backend was not enough for me, I also built a CLI for other users.** I wanted anyone to be able to spin up their own site with just a few commands.

This is the story of how I built that user experience using two incredible Rust crates: `clap` for the command-line interface and `askama` for templating.

### Giving AxumPress a Voice with `clap`

A good CLI feels like a conversation. To achieve this, I turned to `clap`, the undisputed king of command-line argument parsing in Rust. It lets you define your entire CLI structure—commands, subcommands, and arguments—using simple Rust structs and derives.

Our CLI needed to do two main things:
1.  **`init`**: A command to set up a new site for a first-time user.
2.  **`serve`**: A command to build the site and preview it locally.

The `init` command was where I could inject some personality. Instead of forcing users to manually edit a `config.toml` file, I built an interactive wizard.

```rust
// A simplified look at our CLI definition in src/cli.rs
#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a new site with some simple questions
    Init,
    /// Build the site and open it locally
    Serve,
}
```

Running `axumpress init` now kicks off a series of prompts asking for your name, tagline, and social links. It even helps you choose a theme that matches your style. Once you're done, it saves everything to `config.toml`, and boom—your site is configured. This friendly onboarding process was a crucial step in making the project feel like a real product.

### From Data to DOM with `askama`

With our data ready and our CLI in place, we needed to solve the final puzzle: rendering the HTML. This is where `askama` comes in. It's a templating engine that feels like it was designed with Rust's best features in mind.

Why Askama? **Type Safety.**

Unlike simple string replacement, Askama checks your templates at *compile time*. It knows that my `post_detail.html` template needs a `Post` struct and a `HeroConfig` struct. If I forget to pass one, or if I try to access a field that doesn't exist (like `post.titel` instead of `post.title`), the program won't even compile. This has saved me from countless "oops, that page is blank" errors.

The process is beautiful:
1.  **Define a Template Struct:** In our `generator.rs` file, we define a struct that holds the data for a specific template.

    ```rust
    #[derive(Template)]
    #[template(path = "post_detail.html")] // Link to the HTML file
    struct PostDetailTemplate<'a> {
        post: &'a Post,
        all_posts: &'a Vec<Post>,
        hero: &'a HeroConfig,
    }
    ```

2.  **Write the HTML Template:** In `templates/post_detail.html`, we write normal HTML but use Jinja2-style syntax to access the data.

    ```html
    <h1>{{ post.front_matter.title }}</h1>
    <div class="post-content">
      {{ post.html_content|safe }}
    </div>
    ```
    The `|safe` filter is critical here—it tells Askama to render the `html_content` as raw HTML, not as plain text.

3.  **Render!** Our `generator.rs` takes the `Vec<Post>` from the markdown processor, loops through it, and for each post, it creates an instance of `PostDetailTemplate` and calls `.render()`. Askama handles the rest, creating a complete HTML file for each post.

### The Final Result: A Deployable `output` Folder

After running the `axumpress serve` command, the magic is complete. The generator creates an `output/` folder containing everything: `index.html` for the homepage, all the CSS and JavaScript assets, and a `blog/` subdirectory with `index.html` for the post list and an individual HTML file for every single post.

This `output` folder is self-contained and ready to be deployed anywhere—Netlify, Cloudflare Pages, GitHub Pages. Just drag and drop (or `git push`), and your site is live.

Of course, the journey isn't over. The CLI can be expanded. I dream of commands like `axumpress add post`, which would prompt you for the title and category and create the markdown file for you. But for now, the core loop is complete: from a mentor's challenge, to a data pipeline, to a fully functional, user-friendly Static Site Generator. And that feels pretty amazing.
