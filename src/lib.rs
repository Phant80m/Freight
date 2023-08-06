pub mod archive;
pub mod flags;
use clap::{Parser, Subcommand};
#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "package")]
    Package {
        #[clap(short, long)]
        tar: bool,
        #[clap(short, long)]
        zip: bool,
        #[clap(value_name = "FILE_OR_DIR")]
        input_items: Vec<String>,
        #[clap(short, long)]
        output: String,
    },
    #[clap(name = "unpackage")]
    Unpackage {
        #[clap(short, long)]
        tar: bool,
        #[clap(short, long)]
        zip: bool,
        #[clap(value_name = "TAR_GZ")]
        input_items: String,
        #[clap(short, long)]
        output: String,
    },
}
