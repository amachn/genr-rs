mod generator;

use regex::Regex;

use std::{env, io, process::exit};

use generator::Generator;

fn main() -> () {
    // enable dev backtraces
    env::set_var("RUST_BACKTRACE", "1"); // NOTE: debugging purposes only

    // create a new generator (it handles CLI argument collection and validation)
    let mut generator = Generator::new();

    // process loop that allows multiple generations
    loop {
        // generate a password and store it in generator.generated, which is accessible via Display
        generator.make();

        println!("{:?}", generator); // NOTE: debugging purposes only
        println!("generated: {}", generator);
        println!("now what?"); // TODO: convert to a print! and io::stdout().flush().unwrap() setup

        // prompt for new input, ensure it is a valid option
        loop {
            // read in input from stdin
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // TODO: allow multiple arguments

            // TODO: optimize, currently exits on length errors but retries on invalid options
            
            // validate the input
            match input.as_str().to_lowercase().trim() {
                "uppercase" | "upper" | "u" => generator.upper = !generator.upper, // flip uppers
                "lowercase" | "lower" | "l" => generator.lower = !generator.lower, // flip lowers
                "numbers" | "num" | "n" => generator.num = !generator.num, // flip numbers
                "symbols" | "sym" |"s" => generator.sym = !generator.sym, // flip symbols
                "make" | "run" | "get" | "new" => (), // make a new password
                "e" | "ex" | "exit" | "q" | "quit" | "close" => exit(0), // quit program
                other @ _ => { // check if it is a length argument, or get a new input
                    // regex explained (it's pretty bad but whatever):
                    // ^ -- this must be the start of the string w/ nothing else before
                    // (?:l|len|length) -- matches either l, len, or length, but doesn't capture
                    // [ \\t]* -- matches infinitely many spaces or tabs
                    // (\\d{1,3}) -- matches and captures at least 1 but no more than 3 digits
                    // $ -- this must be the end of the string w/ nothing else after
                    let re = Regex::new("^(?:l|len|length)[ \\t]*(\\d{1,3})$").unwrap();
                    if let Some(len) = re.captures(other) {
                        // idx 0 is the full match, idx 1 will be the requested length
                        generator.length = len.get(1).unwrap().as_str().parse::<u8>().unwrap();
                        match generator.validate() {
                            Ok(_) => (),
                            Err(error) => {
                                println!("{}", error);
                                exit(1);
                            },
                        };
                    } else {
                        println!("that's not an option! try again bozo: ");
                        continue;
                    }
                },
            };
            break;
        }
    }
}
