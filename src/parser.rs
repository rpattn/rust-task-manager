use clap::Parser;
use clap::Subcommand;
use std::str::FromStr;

use crate::tasks::task::Priority;
use crate::tasks::task::Status;
use crate::tasks::taskstore::GetBy;
use crate::tasks::taskstore::SortOrder;
use crate::tasks::taskstore::TaskField;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub config_file: Option<String>,
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl FromStr for GetBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_uuid = uuid::Uuid::from_str(s);
        if let Ok(uuid) = as_uuid {
            return Ok(GetBy::ByUuid { uuid });
        }
        let as_usize = usize::from_str(s);
        if let Ok(index) = as_usize {
            Ok(GetBy::ByIndex { index })
        } else {
            Err(String::from("Error parsing id"))
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    List {
        #[arg(long)]
        page: Option<usize>,
        #[arg(long)]
        size: Option<usize>,
        #[arg(long)]
        sort: Option<TaskField>,
        #[arg(long)]
        order: Option<SortOrder>,
        #[arg(long)]
        filter: Option<TaskField>,
        #[arg(long, short)]
        value: Option<String>,
    },
    Get {
        id: Option<GetBy>,
    },
    Add {
        name: String,
        #[arg(short, long)]
        priority: Option<Priority>,
    },
    Edit {
        id: GetBy,
        #[arg(long, short)]
        title: Option<String>,
        #[arg(long, short)]
        priority: Option<Priority>,
        #[arg(long, short)]
        status: Option<Status>,
    },
    Remove {
        id: Option<GetBy>,
        #[arg(short, long)]
        last: bool,
    },
    Clear {
        #[arg(short, long)]
        force: bool,
    },
    Complete {
        id: GetBy,
    },
    Config {
        #[arg(long)]
        tasks_filename: Option<String>,
        #[arg(long)]
        config_filename: Option<String>,
        #[arg(long)]
        page: Option<usize>,
        #[arg(long)]
        size: Option<usize>,
        #[arg(long)]
        sort: Option<TaskField>,
        #[arg(long)]
        order: Option<SortOrder>,
        #[arg(long)]
        filter: Option<TaskField>,
        #[arg(long, short)]
        value: Option<String>,
        #[arg(long)]
        clear_filter: bool,
    },
    Info,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
