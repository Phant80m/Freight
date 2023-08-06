use crate::archive::FreightCompress;
use crate::{Args, Commands};
use std::io;
pub fn handle_args(arg: Args) -> Result<(), io::Error> {
    match arg.subcommand {
        Commands::Package {
            tar,
            zip,
            input_items,
            output,
        } => match (tar, zip) {
            (true, true) => panic!("Cannot use two arguments!"),
            (false, true) => {
                FreightCompress::into_zip(input_items, output)?;
            }
            (true, false) => {
                FreightCompress::into_tar(input_items, output)?;
            }
            (false, false) => unimplemented!(),
        },
        Commands::Unpackage {
            tar,
            zip,
            input_items,
            output,
        } => match (tar, zip) {
            (true, true) => panic!("Cannot use two arguments!"),
            (false, true) => {
                FreightCompress::from_zip(input_items, output)?;
            }
            (true, false) => {
                FreightCompress::from_tar(input_items, output)?;
            }
            (false, false) => unimplemented!(),
        },
    }
    Ok(())
}
