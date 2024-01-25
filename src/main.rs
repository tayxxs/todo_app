use clap::{Arg, App};
fn main() {
    let matches = App::new("My Program")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    // Gets a value for input if supplied by user
    let input = matches.value_of("input").unwrap_or("default.txt");
    println!("Using input file: {}", input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e., 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}
