use std::fs;

use clap::Parser;

use crate::utils::cli::Opts;

pub mod utils;

fn main() {
    // Get options on CLI command
    let opts = Opts::parse();
    let Opts { title } = opts;

    let file_path = format!("books/{}.txt", title.to_lowercase());

    println!("File Path: {}", &file_path);
    let book_string = fs::read_to_string(file_path);

    match book_string {
        Ok(value) => println!("The book you chose is :{}", value),
        Err(e) => println!("Error: {}", e),
    }
}
