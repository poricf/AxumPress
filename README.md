# AxumPress

A command-line first Static Site Generator for building personal portfolios and blogs in Rust.

AxumPress is a CLI tool designed to help you build and maintain a personal portfolio site without ever leaving your terminal. It uses the [Askama](https://github.com/djc/askama) templating engine for type-safe HTML generation and the [Axum](https://github.com/tokio-rs/axum) web framework for its live-preview server. The entire workflow, from setup to deployment, is driven by simple commands.

## Features

- **CLI-Driven Setup**: Use `axumpress init` to interactively set up your site's configuration. No manual file editing required to get started.
- **Static Site Generation**: Compiles your configuration and Askama templates into a clean, static `output/` directory.
- **Type-Safe Askama Templates**: Prevents broken pages at compile time and keeps your presentation logic clean and modular.
- **Live Preview Server**: Use `axumpress serve` to build your site and preview it locally with a high-performance Axum server.
- **Deployment Ready**: The generated `output/` folder can be deployed to any static hosting provider (GitHub Pages, Cloudflare Pages, Netlify, etc.).

## Usage

The entire user flow is handled through the command line.

#### Prerequisites
Ensure you have the Rust toolchain (including `cargo`) installed.

### Step 1: Initialize Your Site

First, clone the repository. Then, run the `init` command to configure your site.

```bash
git clone https://github.com/yourusername/axumpress
cd axumpress
cargo run -- init
```

This command will launch an interactive setup wizard. It will ask for your name, tagline, social links, and other details, then save them to a new `config.toml` file in the project root.

### Step 2: Build & Preview Your Site

Once your `config.toml` is created, you can build and serve your site locally.

```bash
cargo run -- serve
```

This command will:
1.  Read your `config.toml`.
2.  Generate all static HTML files into the `/output` directory.
3.  Start a local web server.

You can now preview your site by navigating to **http://localhost:3000** in your web browser.

### Step 3: Deploy

To deploy your site, simply upload the entire contents of the `output` directory to your favorite static hosting provider.

## Project Vision & Roadmap

This is just the beginning! The long-term vision for AxumPress is to be a fully-featured, CLI-driven SSG. Upcoming features include:

- **Content Management**: `axumpress add post`, `add project`, and `add experience` commands to add new content from the CLI.
- **Full Blog Support**: Complete markdown processing for a list of blog posts and individual post pages.
- **Theme Support**: Easily switch between CSS themes via the CLI.
- **More Sections**: Add and manage Education, Skills, and Contact sections from the configuration.
