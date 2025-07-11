use serde::Deserialize;
use std::{fs, io, path::Path};
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: String,
    pub template: String,
    pub home_template: String,
    pub code_theme: String,
}

pub fn read_config(path: &str) -> io::Result<Config> {
    let raw = fs::read_to_string(path)?;
    let config: Config = from_str(&raw)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(config)
}

pub struct Paths {
    pub css: String,
    pub content_template: String,
    pub home_template: String,
}

impl Config {
    pub fn build_paths(&self) -> Paths {
        Paths {
            css: format!("templates/styles-{}.css", self.theme),
            content_template: format!("templates/{}.html", self.template),
            home_template: format!("templates/{}.html", self.home_template),
        }
    }

    pub fn copy_css_if_exists(&self, css_path: &str) -> io::Result<()> {
        if Path::new(css_path).exists() {
            fs::copy(css_path, "output/css/style.css")?;
        } else {
            eprintln!("⚠️ CSS not found: {}", css_path);
        }
        Ok(())
    }

    pub fn copy_about_page_if_exists(&self) -> io::Result<()> {
        let src = "templates/about_template.html";
        let dst = "output/about/about.html";
        if Path::new(src).exists() {
            fs::copy(src, dst)?;
        } else {
            eprintln!("❌ About page not found. Skipping.");
        }
        Ok(())
    }

    pub fn load_template(&self, path: &str) -> io::Result<String> {
        fs::read_to_string(path).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to read template: {}", e))
        })
    }
}
