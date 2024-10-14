use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Demo One")
        .version("1.0")
        .author("Jonathan Reeves")
        .about("Demonstration One")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file path")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output file path")
                .required(true),
        )
        .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("Input file is required");

    let output = matches
        .get_one::<String>("output")
        .expect("Output file is required");

    println!("Given input file: {}", input);
    println!("Given output file: {}", output);
}
