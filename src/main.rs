use crossterm::terminal;
use ratatui::crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::io;

mod app;
use app::*;
mod jade;
mod ui;

fn main() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let terminal =
        Terminal::new(CrosstermBackend::new(io::stderr())).expect("failed to connect to stderr");
    execute!(io::stderr(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let mut jade = App::init()?;

    match jade.run_app(terminal) {
        Ok(_good_job) => (),
        Err(err) => println!("{err:}"),
    }

    execute!(io::stderr(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    ratatui::restore();

    Ok(())
}
