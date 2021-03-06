use std::borrow::BorrowMut;
use std::io::Write;

use clap::Parser;
use rand::{rngs::OsRng, RngCore, prelude::SliceRandom, Rng};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

    /// Disables color output and carats for non-terminal generators
    #[clap(short = 'N', long)]
    no_term: bool,
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
    }
    
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Red))
        .set_bold(true);

    // Prevent extra info when outputting to non-terminal caller
    if args.no_term { 
        println!("{}", words.concat());            
        return 
    }
    
    for word in &words {
        stdout.set_color(&spec.clone()).unwrap();
        write!(&mut stdout, "{}", &word[..1]).unwrap();
        stdout.reset().unwrap();
        write!(&mut stdout, "{}", &word[1..]).unwrap();
    }
    write!(&mut stdout, "\n").unwrap();

    let indicators = words.iter()
        .map(|w| {
            format!("^{}", (1..w.len()).map(|_| " ".to_string()).collect::<String>())
        })
        .collect::<String>();
    println!("{}", indicators);
    println!("Length: {}", words.concat().len());
    if args.pass_info {
        let len: u32 = dict.len().try_into().unwrap();
        let len_f: f64 = len.try_into().unwrap();
        println!("Entropy Estimates:\n\tknown method: {} bits",len_f.powi(words.len().try_into().unwrap()).log2().floor());
        println!("\n\tbruteforce:");
        println!("\t\tlowercase: {} bits", (26_f64).powi(words.concat().len().try_into().unwrap()).log2().floor());
        println!("\t\tmixedcase: {} bits", (52_f64).powi(words.concat().len().try_into().unwrap()).log2().floor());
        println!("\t\talphanum: {} bits", (62_f64).powi(words.concat().len().try_into().unwrap()).log2().floor());
        println!("\t\talpha+symbol: {} bits", (82_f64).powi(words.concat().len().try_into().unwrap()).log2().floor());
    }
}


