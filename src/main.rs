mod snake;

use std::io;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let stdout = io::stdout();
    match opts {
        Opts::Snake => snake::run(stdout),
    }
}

#[derive(StructOpt)]
enum Opts {
    Snake,
}
