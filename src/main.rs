

pub mod api;
pub mod host;
mod header;
use cliclack::*;
use header::{rainbow_flash_effect, print_animated_rustracer_banner, print_goodbye_banner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rainbow_flash_effect();
    print_animated_rustracer_banner();

    let _ = intro("Mode Selection");
    let mode: &str = select("Select a mode")
    .item("host", "Host a game", "You will be the host of the game")
    .item("join", "Join a game", "Join an existing game")
    .item("cancel", "Cancel" ,"Don't leave me ):")
    .interact()?;

    match mode {
        "host" => host::host().await?,
        "join" => println!("Joining a game is not implemented yet"),
        _ => {
            print_goodbye_banner();
        }
    }
    Ok(())
}


