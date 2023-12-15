extern crate reqwest;
extern crate serde_json;
extern crate rand;

use rand::{Rng, seq::SliceRandom};
use reqwest::header::{HeaderMap, AUTHORIZATION, HeaderValue};
use serde_json::Value;
use std::{error::Error};
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
   spinner.stop("Fetched Snippets");
    outro("Finished Github Snippets").expect("Failed to print outro");
    Ok(snippets)
}

async fn get_random_snippet(token: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);

    let repo_list = get_repos(&client, &headers).await?;
    let selected_repo = repo_list.choose(&mut rand::thread_rng()).ok_or("No repo found")?;

    let file_list = get_files(&client, &headers, selected_repo).await?;
    let selected_file = file_list.choose(&mut rand::thread_rng()).ok_or("No file found")?;

    let code_snippet = get_file_content(&client, selected_file).await?;
    println!("Random Code Snippet:\n{}", code_snippet);

    Ok(code_snippet)
}

async fn get_repos(client: &reqwest::Client, headers: &HeaderMap) -> Result<Vec<String>, Box<dyn Error>> {
    let repos_response = client.get("https://api.github.com/repositories")
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    let repos: Value = serde_json::from_str(&repos_response)?;
    let repo_urls = repos.as_array().unwrap()
        .iter()
        .filter_map(|r| r["url"].as_str().map(String::from))
        .collect();

    Ok(repo_urls)
}

async fn get_files(client: &reqwest::Client, headers: &HeaderMap, repo_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let files_response = client.get(format!("{}/contents/", repo_url))
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    let files: Value = serde_json::from_str(&files_response)?;
    let file_urls = files.as_array().unwrap()
        .iter()
        .filter_map(|f| f["download_url"].as_str().map(String::from))
        .collect();

    Ok(file_urls)
}

async fn get_file_content(client: &reqwest::Client, file_url: &str) -> Result<String, Box<dyn Error>> {
    let content = client.get(file_url)
        .send()
        .await?
        .text()
        .await?;

    Ok(content)
}
