use askama::Template; 
use crate::config::HeroConfig; // 
use std::{fs, io, path::Path};
use crate::config::read_config;





#[derive(Template)]
#[template(path = "home.html")] // <-- This tells Askama to use templates/home.html
struct HomeTemplate<'a> {
    // We pass a reference to the hero config
    hero: &'a HeroConfig,
}

pub fn create_output_folders() -> io::Result<()> {
    let folders = ["output", "output/about", "output/content", "output/css" , "output/js"];
    for folder in folders {
        if !Path::new(folder).exists() {
            fs::create_dir(folder)?;
        }    
    }    
    Ok(())
}    

pub fn generate_home_page(hero_config: &HeroConfig) -> io::Result<()> {
    let template = HomeTemplate { hero: hero_config };

    let html_content = template.render().map_err(|e| { // This line will now work
        io::Error::new(io::ErrorKind::Other, format!("Failed to render template: {}", e))
    })?;

    fs::write("output/index.html", html_content)
}



pub fn build_site() -> io::Result<()> {
    println!("Reading configuration...");
    let config = read_config("config.toml")?;
    let paths = config.build_paths();

    println!("Preparing output directories...");
    create_output_folders()?;

    println!("Copying assets...");
    config.copy_css_if_exists(&paths.css)?;
    config.copy_js_if_exists()?;
    config.copy_about_page_if_exists()?;

    println!("Generating homepage...");
    generate_home_page(&config.hero)?;

    println!("\n✅ Site built successfully!");
    Ok(())
}
