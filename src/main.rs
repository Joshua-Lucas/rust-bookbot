use clap::Parser;

use crate::utils::cli::Opts;

pub mod utils;

fn main() {
    // Get options on CLI command
    let opts = Opts::parse();
    let Opts { title } = opts;

    println!("The book you chose is :{}", title);
}
