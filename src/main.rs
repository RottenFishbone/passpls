use std::borrow::BorrowMut;

use clap::Parser;
use rand::{rngs::OsRng, RngCore, prelude::SliceRandom, Rng};

#[cfg(feature = "style")]
use std::io::Write;
#[cfg(feature = "style")]
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// A path to a dictionary file to use
    #[clap(short, long = "dict")]
    dict_path: Option<String>,
    
    /// Do not print the output to stdout
    ///
    /// Provided by the 'style' feature
    #[clap(short = 'H', long)]
    #[cfg(feature = "style")]
    hidden: bool,
    
    /// Copies the result to the system clipboard. 
    ///
    /// Multiple passwords will be newline delimited.
    /// Provided by the 'clipboard' feature.
    #[clap(short, long)]
    #[cfg(feature = "clipboard")]
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

    /// Outputs some dictionary statistics and implications
    #[clap(long)]
    dict_info: bool,

    /// Outputs the contents of the dictionary to stdout
    #[clap(long)]
    dump_dict: bool,

    /// Disables color output and carats for piping/interop
    ///
    /// Provided by the 'style' feature 
    #[clap(short = 'N', long)]
    #[cfg(feature = "style")]
    no_style: bool,
}

fn main() {
    // Load CLI arguments
    let args = Args::parse();

    // Load our dictionary of word candidates
    let dict: Vec<String>;
    if let Some(_dict_path) = args.dict_path {
        dict = Vec::new();
    }
    else{
        let dict_str = include_str!("../assets/dict.txt");
        dict = dict_str.lines()
            .skip_while(|line| !line.eq_ignore_ascii_case("---"))
            .skip(1)
            .map(|line| String::from(line))
            .collect();
    }
    
    if args.dump_dict {
        for word in dict.iter() {
            println!("{}", word);
        }
        return
    }
    
    // Seed the RNG (OsRng is cryptographically secure)
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    // TODO optionally combine with seeding from random.org
    
    let mut generated: Vec<String> = Vec::new();
    for _ in 0..args.number {
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
        
        // Output result to stdout in desired manner
        if cfg!(feature = "style") { #[cfg(feature = "style")] {
            // Skip printing if output is hidden
            if args.hidden { continue }

            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            let mut spec = ColorSpec::new();
            spec.set_bold(true);

            if !args.no_style {
                // Print words with bolded initial letters
                for word in &words {
                    stdout.set_color(&spec.clone()).unwrap();
                    write!(&mut stdout, "{}", &word[..1]).unwrap();
                    stdout.reset().unwrap();
                    write!(&mut stdout, "{}", &word[1..]).unwrap();
                }
                write!(&mut stdout, "\n").unwrap();
                
                // Print carats
                let indicators = words.iter()
                    .map(|w| {
                        format!("^{}", (1..w.len()).map(|_| " ".to_string()).collect::<String>())
                    })
                .collect::<String>();
                println!("{}", indicators);
            }
            else {
                // Print unstyled output
                println!("{}", words.concat());
            }
        }}
        else {
            // Compiled without styling, simlpy print unstyled
            println!("{}", words.concat());
        }

        generated.push(words.concat());
    }
    
    // Copy to clipboard, if desired
    #[cfg(feature = "clipboard")]
    if args.copy {
        // Determine the clipboard context from the host's display server
        copypasta_ext::display::DisplayServer::select()
            .try_context()
            .expect("Failed to create a clipboard context.")
            .set_contents(generated.join("\n"))
            .expect("Failed to set the contents of your clipboard");
    }
}


