use clap::Parser;

#[derive(Parser)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "Tomoya Kashifuku",
    about = "Super awesome sample RPN calculator",
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE", default_value = "default.txt")]
    formula_file: String,
}

fn main() {
    let opts = Opts::parse();

    println!("File specified: {}", opts.formula_file);
    println!("Is verbosity specified?: {}", opts.verbose)
}
