use clap::{Parser, Subcommand};
use clap::builder::Str;
use dialoguer::console::Key;

#[derive(Parser, Debug)]
#[command(name = "qcli")]
#[command(author = "Gamil <Github:zezogamil")]
#[command(version = "1.0")]
#[command(about = "Encrypt and Decrypt files tool created with Rust", long_about = None)]

pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encrypt {
        #[arg(short, long)]
        path: String,
    },
    Decrypt {
        #[arg(short, long)]
        path: String,

        #[arg(short, long)]
        key: String,
    }
}

