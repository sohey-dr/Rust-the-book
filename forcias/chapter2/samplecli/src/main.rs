use clap::{App, Arg};

fn main() {
    let matches = App::new("My PRN program")
        .version("1.0")
        .author("Your name")
        .about("Super awesome sample PRN calculator")
        .arg(
            Arg::new("formula_file")
                .about("Formulas written in PRN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("output_file")
                .about("Output file")
                .value_name("FILE")
                .index(2)
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("Using formula file: {}", file),
        None => println!("No formula file"),
    }

    let verbose = matches.is_present("verbose");
    println!("Verbose: {}", verbose);
}