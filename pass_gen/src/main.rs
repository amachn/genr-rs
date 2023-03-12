mod generator;

use regex::Regex;

use std::{
    env,
    io::{
        Write, 
        stdin,
        stdout
    }, 
    process::exit,
};

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
        println!("generated: {}\n", generator);
        print!("input an operation: ");
        stdout().flush().unwrap();

        // prompt for new input, ensure it is a valid option
        loop {
            // read in input from stdin
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            // TODO: allow multiple arguments
            
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
                    // ([4-9]|[1-5][0-9]|6[0-4]) -- matches and captures digits from 4-64
                    // $ -- this must be the end of the string w/ nothing else after
                    let re = Regex::new("^(?:l|len|length)[ \\t]*([4-9]|[1-5][0-9]|6[0-4])$").unwrap();
                    if let Some(len) = re.captures(other) {
                        // idx 0 is the full match, idx 1 will be the requested length
                        generator.length = len.get(1).unwrap().as_str().parse::<u8>().unwrap();
                    } else {
                        print!("that's not an option! try again bozo: ");
                        stdout().flush().unwrap();
                        continue;
                    }
                },
            };
            break;
        }
        println!();
    }
}
