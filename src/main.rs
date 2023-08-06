use clap::Parser;
use freight::flags::{handle_args, Args};
use std::io;
fn main() -> Result<(), io::Error> {
    let arg = Args::parse();
    handle_args(arg)?;

    Ok(())
}
