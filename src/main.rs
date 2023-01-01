use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
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

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let reader = stdin().lock();
        run(reader, opts.verbose)
    }
}
