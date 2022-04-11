use clap::Parser;

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
    let _args = Args::parse();
}


