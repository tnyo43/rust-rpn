use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;

use samplecli::rpn::RpnCalculator;

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
    formula_file: Option<PathBuf>,
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
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
