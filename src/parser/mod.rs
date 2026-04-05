use clap::Parser;
use clap::Subcommand;
use std::str::FromStr;
use uuid::Uuid;

use crate::tasks::task::Priority;
use crate::tasks::task::Status;
use crate::tasks::taskstore::GetBy;
use crate::tasks::taskstore::IntoGetBy;
use crate::tasks::taskstore::TaskField;
use crate::tasks::taskstore::SortOrder;

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
            Ok(IdArg::Index { index })
        } else {
            Err(String::from("Error parsing id"))
        }
    }
}

// Only allow user to get by unique fields, preserves internal full GetBy
impl From<IdArg> for GetBy {
    fn from(id: IdArg) -> Self {
        match id {
            IdArg::Index { index } => GetBy::ByIndex(index),
            IdArg::Uuid { uuid } => GetBy::ByUuid(uuid),
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
        id: Option<IdArg>,
    },
    Add {
        name: String,
        #[arg(short, long)]
        priority: Option<Priority>,
    },
    Edit {
        id: IdArg,
        #[arg(long, short)]
        title: Option<String>,
        #[arg(long, short)]
        priority: Option<Priority>,
        #[arg(long, short)]
        status: Option<Status>,
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
        id: IdArg,
    },
}

pub fn get_args() -> Cli {
    Cli::parse()
}

impl IntoGetBy for IdArg {
    fn into_get_by(self) -> GetBy {
        GetBy::from(self)
    }
}
