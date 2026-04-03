use clap::Parser;
use clap::Subcommand;
// use uuid::Uuid;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Get {
        id: Option<usize>,
    },
    Add {
        name: String,
    },
    Remove {
        id: Option<usize>,
        #[arg(short, long)]
        last: bool,
    },
    Clear {
        #[arg(short, long)]
        force: bool,
    },
    Complete {
        id: Option<usize>,
    },
}

pub fn get_args() -> Cli {
    Cli::parse()
}
