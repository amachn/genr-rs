mod cli;
mod generator;

use clap::Parser;

use std::{io, process};

use cli::Cli;
use generator::Generator;

fn main() {
    // parse CLI arguments
    let args = Cli::parse();

    // validate that the passed arguments meet the requirements outlined in Cli::validate
    match args.validate() {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        },
    };

    // create a new generator instance with the validated arguments
    let mut generator = Generator::new(args);

    // process loop that allows multiple generations
    loop {
        // generate a password and store it in generator.generated, which is accessible via Display
        generator.make();

        println!("generated: {}", generator);
        println!("now what? ");

        // prompt for new input, ensure it is a valid option
        loop {
            // read in input from stdin
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // validate that it is an option
            match input.as_str().to_lowercase().trim() {
                "uppercase" | "upper" | "u" => generator.upper = !generator.upper,
                "lowercase" | "lower" | "l" => generator.lower = !generator.lower,
                "numbers" | "num" | "n" => generator.num = !generator.num,
                "symbols" | "sym" |"s" => generator.sym = !generator.sym,
                "make" | "run" | "get" | "new" => break,
                "e" | "ex" | "exit" | "q" | "quit" | "close" => process::exit(0),
                other @ _ => {
                    // here we will use regex to determine if it is a length argument or invalid
                    println!("that's not an option! try again bozo: ")
                },
            };
        }
    }
}
