use colored::*;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Citation {
    citation: String,
    style_fullname: String,
    style_shortname: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    citations: Vec<Citation>,
}

fn print_gradient() {
    let text = "
██████╗ ██╗██████╗  ██████╗██╗     ██╗
██╔══██╗██║██╔══██╗██╔════╝██║     ██║
██████╔╝██║██████╔╝██║     ██║     ██║
██╔══██╗██║██╔══██╗██║     ██║     ██║
██████╔╝██║██████╔╝╚██████╗███████╗██║
╚═════╝ ╚═╝╚═════╝  ╚═════╝╚══════╝╚═╝
";

    let start_color = (255, 0, 0); // Red
    let end_color = (255, 255, 0); // Yellow

    for (i, line) in text.lines().enumerate() {
        let t = i as f32 / (text.lines().count() as f32 - 1.0);
        let r = lerp(start_color.0, end_color.0, t);
        let g = lerp(start_color.1, end_color.1, t);
        let b = lerp(start_color.2, end_color.2, t);

        println!("{}", line.truecolor(r, g, b));
    }
}

fn lerp(start: u8, end: u8, t: f32) -> u8 {
    (start as f32 + t * (end as f32 - start as f32)) as u8
}

fn print_menu() {
    println!("Welcome to bibCLI!");
    println!("Enter the link(s) you wish to cite (Separated by comma for multiple):");
    print!("> ");
}

fn main() {
    print_gradient();
    print_menu();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Split input into multiple links
    let links: Vec<&str> = input.trim().split(',').map(|link| link.trim()).collect();

    // Ask for the desired citation format
    println!("Enter the citation format you want (e.g., APA, Harvard, MLA):");
    let mut format_input = String::new();
    io::stdin()
        .read_line(&mut format_input)
        .expect("Failed to read line");
    let user_format = format_input.trim().to_lowercase(); // Convert to lowercase for consistent comparison

    // Map user-friendly format names to API short names
    let format_map = create_format_map();

    // Check if the entered format is valid
    if let Some(desired_format) = format_map.get(&user_format) {
        // Make API requests for each link
        request(links, desired_format);
    } else {
        println!("Invalid format entered. Please try again with a valid format (e.g., APA, Harvard, MLA).");
    }
}

fn create_format_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("apa".to_string(), "apa".to_string());
    map.insert("harvard".to_string(), "harvard1".to_string());
    map.insert("mla".to_string(), "modern-language-association-with-url".to_string());
    map.insert("chicago".to_string(), "chicago-author-date".to_string());
    map.insert("vancouver".to_string(), "vancouver".to_string());
    map.insert("nature".to_string(), "nature".to_string());
    map
}

#[tokio::main]
async fn request(links: Vec<&str>, desired_format: &str) -> Result<(), Error> {
    let api = "https://api.citeas.org/product/";

    let mut grouped_citations: HashMap<String, Vec<String>> = HashMap::new();

    for link in links {
        let url = format!("{}{}", api, link);

        let response = reqwest::get(&url).await?.text().await?;

        let response_data: ResponseData = serde_json::from_str(&response).expect("Failed to parse JSON");

        for citation in response_data.citations {
            grouped_citations
                .entry(citation.style_shortname.clone())
                .or_insert(Vec::new())
                .push(citation.citation.clone());
        }
    }

    // Display citations in the desired format
    match grouped_citations.get(desired_format) {
        Some(citations) => {
            println!("Citations in {} format:", desired_format);
            for citation in citations {
                println!("{}", citation);
            }
        }
        None => println!("No citations found in the {} format.", desired_format),
    }

    Ok(())
}
