use fancy_regex::Regex;
use std::str;

use clap::Parser;

/// Advent 2015-10
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the part to solve
    #[clap(short, long)]
    part: u8,
}

/// An iterator which counts from one to five
struct Code {
    code: String,
}
impl Code {
    fn requirement_1(word: &[u8]) -> bool {
        for (place, x) in word.iter().enumerate() {
            if place <= word.len() - 3 {
                if word[place + 1] == x + 1 && word[place + 2] == x + 2 {
                    return true;
                }
                continue;
            }
            continue;
        }
        return false;
    }
    fn requirement_2(word: &[u8]) -> bool {
        !word.iter().any(|&x| x == 105 || x == 111 || x == 108)
    }
    fn requirement_3(word: &[u8]) -> bool {
        let word = str::from_utf8(word).unwrap();
        let re = Regex::new(r"(.)\1.*(.)\2").unwrap();
        re.is_match(word).unwrap()
    }

    fn requirements(word: &[u8]) -> bool {
        Code::requirement_1(word) && Code::requirement_2(word) && Code::requirement_3(word)
    }

    fn next(&mut self) {
        let word = unsafe { self.code.as_bytes_mut() };
        let mut new_word = vec![0; 8];

        while !Self::requirements(&word) {
            new_word.clone_from_slice(&word);
            for (place, &letter) in &mut new_word.iter().enumerate().rev() {
                if letter <= 121 && letter >= 97 {
                    word[place] = letter + 1;
                    break;
                }
                word[place] = 97;
                continue;
            }
        }
        self.code = str::from_utf8(word).unwrap().to_string();
    }
}

fn main() {
    let args = Args::parse();

    match args.part {
        1 => {
            let a = String::from("hxbxwxba");

            let mut code = Code { code: a };
            code.next();
            println!("{}", code.code);
        }
        2 => {
            let a = String::from("hxbxxzaa");
            let mut code = Code { code: a };
            code.next();
            println!("{}", code.code);
        }
        _ => eprintln!("Invalid part"),
    }
}
