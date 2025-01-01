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
        Ok(value) => println!("There are {} words in {}.", word_count(value), title),
        Err(e) => println!("Error: {}", e),
    }
}

// Counts to number of words in the given book.
fn word_count(text: String) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let test_cases = vec![
            ("Testing Three Words", 3),
            ("Testing Puncuation.", 2),
            (" Testing whitespace ", 2),
            (
                "This is line one,
                 This is line two.",
                8,
            ),
            ("Testing New/n Test", 3),
        ];
        for (text, expected) in test_cases.iter() {
            assert_eq!(
                word_count(text.to_string()),
                *expected,
                "Test failed for input: '{}', expected: {}",
                text,
                expected
            );
        }
    }
}
