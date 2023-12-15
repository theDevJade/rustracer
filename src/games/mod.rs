use std::{io::{stdout, self}, time::{Instant, Duration}, error::Error, vec, thread};

use crossterm::{terminal::{self, ClearType, enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, cursor, style::{SetForegroundColor, Print, Color}, event::{Event, KeyCode, read, EnableMouseCapture, DisableMouseCapture}};
use ratatui::{backend::CrosstermBackend, Terminal};

use self::render::{App, run_app};
mod render;

pub struct Game {
   pub id: String,
    pub port: u16,
    pub snippets: Vec<String>,
    pub players: Vec<Player>,
    pub time: i64,
    duration: Duration,
}

pub struct Player {
    pub name: String,
    wpm: u16,
    pub score: u16,
    pub finished: bool,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            wpm: 0,
            score: 0,
            finished: false,
        }
    }
}


impl Game {
    pub fn new(id: String, port: u16, snippets: Vec<String>, host: Player, time: i64) -> Game {
        Game {
            id,
            port,
            snippets,
            players: vec![host],
            time,
            duration: Duration::from_secs(time as u64),
        }
    }

   pub async fn start(&mut self)-> Result<(), Box<dyn Error>> {
       render(self).await?;
       Ok(())
    }
}

async fn render(game: &mut Game) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app, game);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}