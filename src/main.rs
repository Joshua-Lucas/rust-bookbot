use std::fs;

use clap::Parser;

use crate::utils::cli::Opts;

mod utils {
    pub mod cli;
}

#[derive(PartialEq, Debug)]
struct Letter {
    name: char,
    count: i32,
}

#[derive(PartialEq, Debug)]
struct Letters {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
    i: i32,
    j: i32,
    k: i32,
    l: i32,
    m: i32,
    n: i32,
    o: i32,
    p: i32,
    q: i32,
    r: i32,
    s: i32,
    t: i32,
    u: i32,
    v: i32,
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

// Method to format the Letters Struct
impl Letters {
    // returns a vec of the fields to be itered
    fn iter_fields(&self) -> Vec<Letter> {
        vec![
            Letter {
                name: 'a',
                count: self.a,
            },
            Letter {
                name: 'b',
                count: self.b,
            },
            Letter {
                name: 'c',
                count: self.c,
            },
            Letter {
                name: 'd',
                count: self.d,
            },
            Letter {
                name: 'e',
                count: self.e,
            },
            Letter {
                name: 'f',
                count: self.f,
            },
            Letter {
                name: 'g',
                count: self.g,
            },
            Letter {
                name: 'h',
                count: self.h,
            },
            Letter {
                name: 'i',
                count: self.i,
            },
            Letter {
                name: 'j',
                count: self.j,
            },
            Letter {
                name: 'k',
                count: self.k,
            },
            Letter {
                name: 'l',
                count: self.l,
            },
            Letter {
                name: 'm',
                count: self.m,
            },
            Letter {
                name: 'n',
                count: self.n,
            },
            Letter {
                name: 'o',
                count: self.o,
            },
            Letter {
                name: 'p',
                count: self.p,
            },
            Letter {
                name: 'q',
                count: self.q,
            },
            Letter {
                name: 'r',
                count: self.r,
            },
            Letter {
                name: 's',
                count: self.s,
            },
            Letter {
                name: 't',
                count: self.t,
            },
            Letter {
                name: 'u',
                count: self.u,
            },
            Letter {
                name: 'v',
                count: self.v,
            },
            Letter {
                name: 'w',
                count: self.w,
            },
            Letter {
                name: 'x',
                count: self.x,
            },
            Letter {
                name: 'y',
                count: self.y,
            },
            Letter {
                name: 'z',
                count: self.z,
            },
        ]
    }

    //sorts letter count in descending order.
    fn sort_letters(&self) -> Vec<Letter> {
        let mut letters = self.iter_fields();

        letters.sort_by(|a, b| b.count.cmp(&a.count));

        return letters;
    }

    // Prints a formatted count in order of most occurrences to least.
    fn print_count(&self) {
        let letters = self.sort_letters();

        letters
            .iter()
            .for_each(|x| println!("The '{}' character was found {} times", x.name, x.count))
    }
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
            println!("{} words in the document.", word_count(&value));
            println!("");
            char_count(&value).print_count();
            println!("--- End report ---");
        }
        Err(e) => println!("Error: {}", e),
    }
}

// Counts to number of words in the given book.
fn word_count(text: &String) -> usize {
    text.split_whitespace().count()
}

// Counts the occurrence of all characters in the book.
fn char_count(text: &String) -> Letters {
    let mut letter_count = Letters {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        g: 0,
        h: 0,
        i: 0,
        j: 0,
        k: 0,
        l: 0,
        m: 0,
        n: 0,
        o: 0,
        p: 0,
        q: 0,
        r: 0,
        s: 0,
        t: 0,
        u: 0,
        v: 0,
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };

    text.chars().for_each(|x| match x.to_ascii_lowercase() {
        'a' => letter_count.a = letter_count.a + 1,
        'b' => letter_count.b = letter_count.b + 1,
        'c' => letter_count.c = letter_count.c + 1,
        'd' => letter_count.d = letter_count.d + 1,
        'e' => letter_count.e = letter_count.e + 1,
        'f' => letter_count.f = letter_count.f + 1,
        'g' => letter_count.g = letter_count.g + 1,
        'h' => letter_count.h = letter_count.h + 1,
        'i' => letter_count.i = letter_count.i + 1,
        'j' => letter_count.j = letter_count.j + 1,
        'k' => letter_count.k = letter_count.k + 1,
        'l' => letter_count.l = letter_count.l + 1,
        'm' => letter_count.m = letter_count.m + 1,
        'n' => letter_count.n = letter_count.n + 1,
        'o' => letter_count.o = letter_count.o + 1,
        'p' => letter_count.p = letter_count.p + 1,
        'q' => letter_count.q = letter_count.q + 1,
        'r' => letter_count.r = letter_count.r + 1,
        's' => letter_count.s = letter_count.s + 1,
        't' => letter_count.t = letter_count.t + 1,
        'u' => letter_count.u = letter_count.u + 1,
        'v' => letter_count.v = letter_count.v + 1,
        'w' => letter_count.w = letter_count.w + 1,
        'x' => letter_count.x = letter_count.x + 1,
        'y' => letter_count.y = letter_count.y + 1,
        'z' => letter_count.z = letter_count.z + 1,
        _ => (),
    });

    return letter_count;
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
                word_count(&text.to_string()),
                *expected,
                "Test failed for input: '{}', expected: {}",
                text,
                expected
            );
        }
    }

    #[test]
    fn test_char_count() {
        let test_cases = vec![
            (
                "abcdefghijklmnopqrstuvwxyz",
                Letters {
                    a: 1,
                    b: 1,
                    c: 1,
                    d: 1,
                    e: 1,
                    f: 1,
                    g: 1,
                    h: 1,
                    i: 1,
                    j: 1,
                    k: 1,
                    l: 1,
                    m: 1,
                    n: 1,
                    o: 1,
                    p: 1,
                    q: 1,
                    r: 1,
                    s: 1,
                    t: 1,
                    u: 1,
                    v: 1,
                    w: 1,
                    x: 1,
                    y: 1,
                    z: 1,
                },
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
                Letters {
                    a: 2,
                    b: 2,
                    c: 2,
                    d: 2,
                    e: 2,
                    f: 2,
                    g: 2,
                    h: 2,
                    i: 2,
                    j: 2,
                    k: 2,
                    l: 2,
                    m: 2,
                    n: 2,
                    o: 2,
                    p: 2,
                    q: 2,
                    r: 2,
                    s: 2,
                    t: 2,
                    u: 2,
                    v: 2,
                    w: 2,
                    x: 2,
                    y: 2,
                    z: 2,
                },
            ),
            (
                "()!#&*.,abcdefghijklmnopqrstuvwxyz",
                Letters {
                    a: 1,
                    b: 1,
                    c: 1,
                    d: 1,
                    e: 1,
                    f: 1,
                    g: 1,
                    h: 1,
                    i: 1,
                    j: 1,
                    k: 1,
                    l: 1,
                    m: 1,
                    n: 1,
                    o: 1,
                    p: 1,
                    q: 1,
                    r: 1,
                    s: 1,
                    t: 1,
                    u: 1,
                    v: 1,
                    w: 1,
                    x: 1,
                    y: 1,
                    z: 1,
                },
            ),
        ];
        for (text, expected) in test_cases.iter() {
            assert_eq!(
                char_count(&text.to_string()),
                *expected,
                "Test failed for input: '{}', expected: {:?}",
                text,
                expected
            );
        }
    }
}
