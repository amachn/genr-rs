mod cli;
mod generator;

use clap::Parser;

use cli::Cli;
use generator::Generator;

fn main() {
    // pass parsed CLI arguments to new Generator instance, then run the generator
    let mut generator = Generator::new(Cli::parse());
    generator.make();
}
