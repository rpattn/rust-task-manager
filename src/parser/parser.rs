use clap::Parser;
use clap::Subcommand;
use std::str::FromStr;
use uuid::Uuid;

use crate::tasks::task::Priority;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Clone, Debug, Copy)]
pub enum IdArg {
    Index { index: usize },
    Uuid { uuid: Uuid },
}

impl FromStr for IdArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_uuid = uuid::Uuid::from_str(s);
        if let Ok(uuid) = as_uuid {
            return Ok(IdArg::Uuid { uuid });
        }
        let as_usize = usize::from_str(s);
        if let Ok(index) = as_usize {
            return Ok(IdArg::Index { index: index });
        } else {
            return Err(String::from("Error parsing id"));
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    Get {
        id: Option<IdArg>,
    },
    Add {
        name: String,
        #[arg(short, long)]
        priority: Option<Priority>,
    },
    Remove {
        id: Option<IdArg>,
        #[arg(short, long)]
        last: bool,
    },
    Clear {
        #[arg(short, long)]
        force: bool,
    },
    Complete {
        id: Option<IdArg>,
    },
}

pub fn get_args() -> Cli {
    Cli::parse()
}
