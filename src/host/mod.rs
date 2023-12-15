use cliclack::{intro, input, select, outro, confirm};

use crate::api::gh_snippet::get_random_snippets;

pub mod port_scanner;

pub async fn host() -> Result<(), Box<dyn std::error::Error>> {
    let _ = intro("Welcome to rustracer! Lets setup you as a host.");

    let name: String = input("Enter your name").interact()?;

    let mode: &str = select("Select a mode")
    .item("typeracer", "TypeRacer", "The Original Mode")
    .item("wip", "WIP, not added", "")
    .interact()?;

    let _ = outro("Finished initial setup");

    let _ = intro("Ports");
    let contin = confirm("In order to host, you need to have open ports, would you like to scan for open ports?").interact()?;
    if contin {
        port_scanner::scan_common_ports();
    }
    let port: u16 = input("Enter a port to host on").interact()?;

    let snippets: Vec<String> = get_random_snippets().await?;
    Ok(())
}