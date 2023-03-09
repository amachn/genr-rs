mod cli;
mod generator;

use clap::Parser;

use std::process;

use cli::Cli;
use generator::Generator;

fn main() {
    // pass parsed CLI arguments to new Generator instance, then run the generator
    let args = Cli::parse();

    match args.validate() {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        },
    };

    let mut generator = Generator::new(args);
    generator.make();
}
