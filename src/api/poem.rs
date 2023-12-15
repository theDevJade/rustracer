use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Poem {
    title: String,
    author: String,
    lines: Vec<String>,
}

pub async fn get_random_poem() -> Result< Vec<String>, Box<dyn Error>> {
    let url = "https://poetrydb.org/random";
    let response = reqwest::get(url).await?;
    println!("{}", response.status());
    let poem = response.json::<Vec<Poem>>().await?;

    let poem = poem.first().unwrap();
    Ok(poem.lines.clone())
}