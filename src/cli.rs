use clap::{Parser, Subcommand};
use crate::config::{Config, HeroConfig};
use crate::generator::build_site;
use crate::server::serve_web;
use rand::Rng;
use std::io::{self, Write};
use std::fs;
use std::{thread, time};

/// CLI definition
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a new site with some simple questions
    Init,
    /// Build the site and open it locally
    Serve,
}

/// Ask user something and get their answer
fn prompt_for_input(prompt_text: &str) -> io::Result<String> {
    print!("{}", prompt_text);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!();
    Ok(input.trim().to_string())
}

/// Roast and hype the user based on their role
fn react_to_role(tagline: &str) {
    let tag = tagline.to_lowercase();
    let (reaction, roast) = if tag.contains("developer") || tag.contains("dev") {
        (
            "💻 Okay coder. Stack Overflow warrior, huh?",
            "Good thing this builder’s gonna make you look clean — even if your CSS is still fighting you."
        )
    } else if tag.contains("engineer") || tag.contains("enginer") || tag.contains("engeneer") {
        (
            "🔧 Big engineer energy. Probably overthinking the folder structure already.",
            "Don’t worry — this site gon’ make you look polished, even if your Git history says otherwise."
        )
    } else if tag.contains("designer") {
        (
            "🎨 Designer spotted! You probably got Figma open right now.",
            "We both know pixels ain’t code — this builder got your back. You’ll look good, promise."
        )
    } else if tag.contains("writer") || tag.contains("author") || tag.contains("journalist") {
        (
            "✍️ Word wizard in the house.",
            "Even if your drafts folder has more stories than your site, this builder gon’ make it look deep."
        )
    } else if tag.contains("student") {
        (
            "📚 Student grind detected. GPA? Optional. Portfolio? Required.",
            "You’re still figuring it out — good thing this site makes you look like you already did."
        )
    } else if tag.contains("freelancer") || tag.contains("consultant") {
        (
            "🧑‍💼 Consultant vibes. Big LinkedIn energy.",
            "Clients want trust — this site gon’ fake it till you make it (but clean tho)."
        )
    } else if tag.contains("artist") || tag.contains("illustrator") {
        (
            "🎨 Artist mode. Probably have 200 layers named ‘Layer 1’.",
            "This site’s your gallery now — it’ll show the world you’re not just vibes. (Mostly.)"
        )
    } else {
        (
            "🌟 You’re unique. I respect it.",
            "Whatever you’re on, this builder gon’ make it look better than it actually is 😎"
        )
    };

    println!("\n{}\n{}\n", reaction, roast);
    thread::sleep(time::Duration::from_millis(800));
}

/// Run the interactive site setup
pub fn init_site() -> io::Result<()> {
    println!("🚀 Yo! Let’s set up your personal site real quick.");
    println!("Few easy questions — I’ll handle the rest. No stress.\n");
    thread::sleep(time::Duration::from_millis(600));

    if fs::metadata("config.toml").is_ok() {
        let overwrite = prompt_for_input("⚠️  You already got a setup here. Wanna start fresh? (y/n): ")?;
        if overwrite.to_lowercase() != "y" {
            println!("👍 Cool cool. Using what you already got.");
            return Ok(());
        }
    }

    let name = prompt_for_input("🙋 What’s your full name? (e.g., Amanuel Fikru): ")?;
    thread::sleep(time::Duration::from_millis(300));

    let tagline = prompt_for_input("📝 What do you do or wanna be known for? (e.g., Designer, Writer, Student): ")?;
    react_to_role(&tagline);

    let image_input = prompt_for_input("🖼️ Where your profile pic at? (e.g., /static/profile.jpg, or leave blank): ")?;
    let image = if image_input.trim().is_empty() {
        println!("🙈 Bro really skipped the profile pic... You get the chimp now.");
        println!("📸 '/static/chimp.jpg' is your face until you get serious 💀\n");
        "/static/chimp.jpg".to_string()
    } else {
        image_input
    };

    thread::sleep(time::Duration::from_millis(400));
    let github = prompt_for_input("🌐 GitHub link? (or leave empty if you ghost): ")?;
    thread::sleep(time::Duration::from_millis(300));
    let linkedin = prompt_for_input("🔗 LinkedIn link? (no stress if you don’t use it): ")?;
    println!();

    let hero_config = HeroConfig {
        name,
        tagline,
        image,
        github_url: github,
        linkedin_url: linkedin,
    };

    let full_config = Config {
        theme: {
            println!("🎨 Pick a vibe for your site:");
            println!("  1) Clean & Light ✨");
            println!("  2) Dark & Bold 🌑");
            println!("(Or press Enter if you can’t decide)\n");

            let choice = prompt_for_input("Pick 1 or 2: ")?;
            match choice.trim() {
                "1" => "porcif".to_string(),
                "2" => "dracula".to_string(),
                "3" => "warm".to_string(),
                "4" => "nature".to_string(),
                "5" => "neon".to_string(),
                "6" => "minimal".to_string(),
                "7" => "retro".to_string(),
                "8" => "dark".to_string(),
                "" => {
                    let themes = ["porcif", "dracula"];
                    let idx = rand::thread_rng().gen_range(0..themes.len());
                    println!("🎲 You ain’t pickin’? Alright — we rollin’ with '{}'.", themes[idx]);
                    themes[idx].to_string()
                }
                _ => {
                    println!("🤷 That ain’t even an option. Goin’ with 'porcif' by default.");
                    "porcif".to_string()
                }
            }
        },
        template: "content_template".to_string(),
        home_template: "home".to_string(),
        code_theme: "atom-one-dark".to_string(),
        hero: hero_config,
    };

    let toml_string = toml::to_string(&full_config)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("config.toml", toml_string)?;

    println!("\n🍳 Portfolio is getting cooked...");
    thread::sleep(time::Duration::from_secs(5));
    println!("✅ That’s it. You cooked.");
    println!("🎉 This site might just be your glow-up fr. Run `cargo run -- serve` to see the magic.\n");
    Ok(())
}

/// Build and start the local server
pub async fn build_and_serve() -> io::Result<()> {
    build_site()?;
    serve_web("output", 3000).await
}
