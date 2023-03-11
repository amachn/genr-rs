mod generator;

use regex::Regex;

use std::{env, io, process};

use generator::Generator;

fn main() {
    // enable dev backtraces
    env::set_var("RUST_BACKTRACE", "1");

    // create a new generator (it handles CLI argument collection and validation)
    let mut generator = Generator::new();

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

            // validate the input
            match input.as_str().to_lowercase().trim() {
                "uppercase" | "upper" | "u" => generator.upper = !generator.upper, // flip uppers
                "lowercase" | "lower" | "l" => generator.lower = !generator.lower, // flip lowers
                "numbers" | "num" | "n" => generator.num = !generator.num, // flip numbers
                "symbols" | "sym" |"s" => generator.sym = !generator.sym, // flip symbols
                "make" | "run" | "get" | "new" => break, // make a new password
                "e" | "ex" | "exit" | "q" | "quit" | "close" => process::exit(0), // quit program
                other @ _ => { // check if it is a length argument, or get a new input
                    // regex explained (it's pretty bad but whatever):
                    // ^ -- this must be the start of the string w/ nothing else before
                    // (?:l|len|length) -- matches either l, len, or length 
                    // [ \\t]* -- matches infinitely many spaces or tabs
                    // \\d{1,3} -- matches at least 1 but no more than 3 digits
                    // $ -- this must be the end of the string w/ nothing else after
                    let re = Regex::new("^(?:l|len|length)[ \\t]*\\d{1,3}$").unwrap();
                    if let Some(len) = re.captures(other) {
                        // process the new password length here
                    } else {
                        println!("that's not an option! try again bozo: ");
                    }
                },
            };
        }
    }
}
