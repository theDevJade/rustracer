use cliclack::{intro, input, select, outro};

use crate::api::gh_snippet::get_random_snippets;

pub async fn host() -> Result<(), Box<dyn std::error::Error>> {
    let _ = intro("Welcome to rustracer! Lets setup you as a host.");

    let name: String = input("Enter your name").interact()?;

    let mode: &str = select("Select a mode")
    .item("typeracer", "TypeRacer", "The Original Mode")
    .item("wip", "WIP, not added", "")
    .interact()?;

    let _ = outro("Finished initial setup");

    let snippets: Vec<String> = get_random_snippets().await?;
    Ok(())
}