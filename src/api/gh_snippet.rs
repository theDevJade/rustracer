extern crate reqwest;
extern crate serde_json;
extern crate rand;

use async_recursion::async_recursion;
use rand::{Rng, seq::SliceRandom};
use reqwest::header::{HeaderMap, AUTHORIZATION, HeaderValue};
use serde_json::Value;
use std::{error::Error, path::Path, fs::{self, File}, fmt::format, io::Write};
use cliclack::*;

pub async fn get_random_snippets() -> Result<Vec<String>, Box<dyn Error>> {
    let _ = intro("Github Snippets");

    let token: String = input("Enter your Github token")
    .required(true)
    .default_input("Your Classic Github Auth Token")
    .interact()?;

    let amount: usize = input("How many snippets do you want?")
    .required(false)
    .default_input("3")
    .interact()?;

    let mut spinner = spinner();
    spinner.start("Fetching Snippets...");
    let mut snippets = vec![get_random_snippet(&token).await?];

    for _ in 1..(amount - 1) {
        snippets.push(get_random_snippet(&token).await?);
    }

    let snippets_dir = Path::new("./snippets");

    if !snippets_dir.exists() {
        fs::create_dir(snippets_dir)?;
    } else {
        fs::remove_dir_all(snippets_dir)?;
        fs::create_dir(snippets_dir)?;
    }

    let comment_markers = vec!["#", "//"];

    let mut index = 1;
    for snippet in snippets.iter() {
        let file_path = snippets_dir.join(format!("snippet-{index}"));
        let mut file = File::create(file_path)?;
    
        'line_loop: for line in snippet.lines() {
            let trimmed_line = line.trim_start();
    
            // Check if the line starts with any of the comment markers
            for marker in &comment_markers {
                if trimmed_line.starts_with(marker) {
                    continue 'line_loop;
                }
            }
    
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?;
        }
    
        index += 1;
    }

   spinner.stop("Fetched Snippets");
    outro(format!("Finished Creating {} Github Snippets", amount)).expect("Failed to print outro");
    Ok(snippets)
}

async fn get_random_snippet(token: &str) -> Result<String, Box<dyn Error>> {
    let url = "https://api.github.com/repositories";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github.antiope-preview+json"));
    headers.insert("Authorization", HeaderValue::from_str(&format!("token {}", token))?);
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));
    headers.insert("User-Agent", HeaderValue::from_static("TypeRacer/Rust"));

    let response = client.get(url)
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    

    let repos: Vec<Value> = serde_json::from_str(&response)?;
    let repo_urls: Vec<String> = repos.into_iter()
        .filter_map(|r| r["html_url"].as_str().map(String::from))
        .collect();

    let selected_repo = repo_urls.choose(&mut rand::thread_rng())
        .ok_or_else(|| "No repository found")?;

    println!("{}",selected_repo);

    let repo_contents = get_random_file_content(&client, &headers, &selected_repo, "").await?;
    Ok(repo_contents)
}

async fn get_repo_contents(client: &reqwest::Client, headers: &HeaderMap, repo_url: &str, path: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut broken_url = repo_url.replace("https://github.com/", "");
    let actual_url: Vec<&str> = broken_url.split("/").collect();
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}",actual_url[0], actual_url[1], path);
    let response = client.get(&url)
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    let contents: Vec<Value> = serde_json::from_str(&response)?;
    Ok(contents)
}
#[async_recursion]
async fn get_random_file_content(client: &reqwest::Client, headers: &HeaderMap, repo_url: &str, current_path: &str) -> Result<String, Box<dyn Error>> {
    let contents = get_repo_contents(client, headers, repo_url, current_path).await?;
    let selected_content = contents.choose(&mut rand::thread_rng())
        .ok_or_else(|| "No content found")?;

    let name = selected_content["name"].clone();

    match selected_content["type"].as_str() {
        Some("file") => {
            
            let filter_extensions = vec![".md", ".ext", "TODO", "Rakefile", ".txt", "README", "LICENSE", ".rdoc", ".png", ".jpeg", ".swf", ".gitignore"];
           for extension in filter_extensions {
               if name.as_str().unwrap().contains(extension) {
                   let _ = get_random_file_content(client, headers, repo_url, "").await;
                   break;
               }
            }
            let file_url = selected_content["download_url"].as_str().ok_or("Missing file URL")?;
            let file_response = client.get(file_url)
                .send()
                .await?
                .text()
                .await?;
            println!("💰 Ye found gooold {}", name.as_str().unwrap());
            Ok(file_response)
        }
        Some("dir") => {
            let new_path = selected_content["path"].as_str().ok_or("Missing directory path")?;
            println!("📦Unpacking {}", new_path);
            get_random_file_content(client, headers, repo_url, new_path).await
        }
        _ => Err("Unknown content type".into())
    }
}
