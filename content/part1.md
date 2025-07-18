---
title: "How I Built This Blog (Part 1): From a Mentor's Challenge to a Data Pipeline"
date: "2025-07-25"
category: "Projects"
tags: ["rust", "ssg", "learning", "mentorship"]
excerpt: "It all started with a challenge from my mentor. 'Build your own blog,' he said. That challenge sparked a journey into Rust that went far beyond just learning. Here’s the story of how this site came to be, starting with the core data pipeline."
featured: true
# image: "/images/a.jpg" # Example image path
---

It all started with a challenge from my mentor, Matthew Harwood (Aurter). When I was starting out, he gave me a simple, yet profound, piece of advice: **"Build your own blog site so that you can write blogs in it."**

It wasn't just about learning to code; it was about building a home for my own thoughts and creating a tool that was truly mine. Initially, I just aimed to learn Rust, but as I started building, I found myself falling in love with the process, the language's power, and the sheer satisfaction of creating something from scratch. This blog is the result of that journey.

This series is the story of AxumPress, the little Rust engine that runs this site. In this first part, we're not even touching HTML yet. We're tackling the most fundamental challenge that my mentor's advice created: How do you turn a folder of simple text files into structured, usable data in memory?

### The First Hurdle: From Text to Data

My first, primitive attempt was just reading a markdown file and crudely injecting its content into an HTML template using `string.replace()`. It worked, but it was ugly and inflexible. How would I get the title? The date? How could I show a *list* of posts on the homepage?

This led me to the three core pillars of the AxumPress data pipeline.

#### Pillar 1: Front-Matter for Metadata (`gray-matter`)

A blog post is more than just its content. It has a title, a publication date, categories, tags... this is metadata. The standard way to handle this in the SSG world is with **front-matter**, a small block of YAML at the very top of your markdown file.

This very post starts with it! The `gray-matter` crate for Rust is a lifesaver here. It takes a raw file and neatly splits it into two parts:
1.  The structured YAML data.
2.  The rest of the file (the markdown body).

#### Pillar 2: The Markdown Engine (`comrak`)

Once we have the markdown body, we need to convert it into HTML. The `comrak` crate is an incredibly powerful and spec-compliant CommonMark parser. We feed it the raw markdown string, and it spits out clean HTML, automatically handling headings, lists, code blocks, and more. We even configured it to automatically add `id` attributes to headings, which is crucial for the "On this page" table of contents to work.

#### Pillar 3: The `Post` Struct (Our Rust Blueprint)

This is where the Rust philosophy really shines. We don't want to pass around loose strings and bits of data. We want structure and safety. So, we define a "blueprint" for what a post is in our system.

```rust
// This is our Rust struct in src/models.rs
pub struct Post {
    pub front_matter: FrontMatter, // The metadata from YAML
    pub slug: String,             // A URL-friendly name, like "how-i-built-this"
    pub html_content: String,     // The final HTML from Comrak
}
```
`FrontMatter` is another struct that holds the title, date, category, etc.

### The Final Pipeline (So Far)

With these tools, our blog extraction process becomes a clean, elegant pipeline:

1.  **Walk the Directory:** The program starts by looking inside the `content/` folder for any file ending in `.md`.
2.  **Read and Parse:** For each file, `gray-matter` splits it into metadata and markdown content.
3.  **Deserialize and Convert:** We deserialize the metadata into our `FrontMatter` struct. Simultaneously, `comrak` converts the markdown content into an HTML string.
4.  **Assemble:** We take all these clean pieces of data—the front-matter, the generated HTML, and a "slug" derived from the filename—and assemble them into our perfect `Post` struct.
5.  **Collect:** Every `Post` struct is pushed into a single `Vec<Post>` (a list, for non-Rustaceans).

At the end of this process, we have a single variable in our program: a perfectly ordered vector containing every single blog post, ready to be displayed.

But how do we get that data onto a webpage? That involves the *other* half of the magic: the generator and the **Askama** templating engine. Askama allows us to write HTML templates that are type-checked by the Rust compiler, preventing a whole class of errors before the site even builds. It's the bridge that will take our `Vec<Post>` and turn it into the beautiful blog you're reading now.

And that's exactly what we'll cover in **Part 2**. Stay tuned!
