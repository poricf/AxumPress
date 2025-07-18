// src/models.rs

use serde::{Deserialize, Serialize};

// This struct MUST exactly match the fields in your markdown front-matter.
#[derive(Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub category: String,
    pub tags: Vec<String>,
    pub excerpt: String,
    pub featured: Option<bool>,
     pub image: Option<String> // `Option<bool>` handles cases where 'featured' is not set.
}

// This is the final struct that our templates will use.
#[derive(Debug, Serialize)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub slug: String, // e.g., "my-first-post"
    pub html_content: String,
    pub reading_time_minutes: u32
}