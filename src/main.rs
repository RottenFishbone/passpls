use std::borrow::BorrowMut;

use clap::Parser;
use rand::{rngs::OsRng, RngCore, prelude::SliceRandom, Rng};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// A path to a dictionary file to use
    #[clap(short, long = "dict")]
    dict_path: Option<String>,
    
    /// Do not print the output to stdout
    #[clap(short = 'H', long)]
    hidden: bool,

    /// Copies the output to the system clipboard. 
    ///
    /// Multiple passwords will be newline delimited.
    #[clap(short, long)]
    copy: bool,

    /// How many passwords to generate
    #[clap(short, long = "count", default_value_t = 1)]
    number: u32,

    /// Maximum length of the generated password(s)
    #[clap(short = 'L', long, default_value_t = 32)]
    max_len: u8,
    /// Minimum length of the generated password(s)
    #[clap(short = 'l', long, default_value_t = 0)]
    min_len: u8,

    /// Maximum number of words in the generated password(s)
    #[clap(short = 'W', long, default_value_t = 3)]
    max_words: u8,
    /// Minumum number of words in the generated password(s)
    #[clap(short = 'w', long, default_value_t = 3)]
    min_words: u8,

    /// Outputs some password statistics and info alonside a generated password
    #[clap(short = 'i', long)]
    pass_info: bool,

    /// Outputs some dictionary statistics and implications
    #[clap(short = 'I', long)]
    dict_info: bool,
}

fn main() {
    // Load CLI arguments
    let args = Args::parse();

    // Seed the RNG (OsRng is cryptographically secure)
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    // TODO combine with seeding from random.org

    // Load our dictionary of word candidates
    let dict: Vec<String>;
    if let Some(_dict_path) = args.dict_path {
        dict = Vec::new();
    }
    else{
        let dict_str = include_str!("../assets/dict.txt");
        dict = dict_str.lines()
            .skip_while(|line| line.eq_ignore_ascii_case("---"))
            .map(|line| String::from(line))
            .collect();
    }
    

    // Choose the number of words to generate
    let num_words: u8;
    if args.min_words == args.max_words {
        num_words = args.max_words;
    }
    else {
        num_words = OsRng.borrow_mut().gen_range(args.min_words..(args.max_words+1));
    }
 

    // Generate a single password
    let mut words: Vec<String> = Vec::new();
    for _ in 0..num_words {
        let word = dict.choose(OsRng.borrow_mut())
            .expect("Failed to select a word from the dictionary");

        words.push(word.to_owned());
        words.push(" ".to_string());
    }
 
    println!("{}", words.concat());
}


