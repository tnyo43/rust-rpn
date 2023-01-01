use clap::{Arg, Command};

fn main() {
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("Tomoya")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("verbose")
                .help("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false)
        )
        .get_matches();
    
    match matches.get_one::<String>("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose: bool = matches.contains_id("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
