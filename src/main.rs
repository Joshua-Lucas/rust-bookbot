use std::fs;

use clap::Parser;

use crate::utils::cli::Opts;

mod utils {
    pub mod cli;
}

mod analytics {
    pub mod core;
}

fn main() {
    // Get options on CLI command
    let opts = Opts::parse();
    let Opts { title } = opts;

    let file_path = format!("books/{}.txt", title.to_lowercase());

    println!("File Path: {}", &file_path);
    let book_string = fs::read_to_string(file_path);

    match book_string {
        Ok(value) => {
            println!("--- Begin report of {} ---", title);
            println!("");
            println!(
                "{} words in the document.",
                analytics::core::word_count(&value)
            );
            println!("");
            analytics::core::char_count(&value).print_count();
            println!("--- End report ---");
        }
        Err(e) => println!("Error: {}", e),
    }
}
