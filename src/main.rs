mod common;
mod snake;

use crossterm::{cursor, queue, terminal};
use std::io;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    queue!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;

    match opts {
        Opts::Snake => snake::run(&mut stdout)?,
    }

    queue!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

#[derive(StructOpt)]
enum Opts {
    Snake,
}
