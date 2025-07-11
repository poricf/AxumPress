use std::{fs, io, path::Path};
use comrak::{markdown_to_html, ComrakOptions};

pub fn process_markdown_files(
    dir: &str,
    html_template: &str
) -> io::Result<Vec<(String, String, String)>> {
    let path = Path::new(dir);
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "'content' directory not found"));
    }

    let mut contents = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let slug = path.file_stem()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid filename"))?
            .to_string_lossy()
            .to_string();

        let (html, preview) = markdown_2_html(path.to_str().unwrap(), html_template)?;
        fs::write(format!("output/content/{}.html", slug), html)?;
        contents.push((slug.clone(), slug, preview));
    }

    Ok(contents)
}

fn markdown_2_html(md_path: &str, html_template: &str) -> io::Result<(String, String)> {
    let markdown = fs::read_to_string(md_path)?;

    let preview = extract_preview(&markdown, 200);

    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.parse.smart = true;
    options.render.unsafe_ = true;
    options.extension.header_ids = Some("".to_string());

    let html_body = markdown_to_html(&markdown, &options);
    let full_html = html_template.replace("{body}", &html_body);

    Ok((full_html, preview))
}

fn extract_preview(content: &str, max_len: usize) -> String {
    let clean = content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");

    let preview = clean.chars().take(max_len).collect::<String>();
    if clean.len() > max_len {
        format!("{}...", preview)
    } else {
        preview
    }
}
