use crate::cli::Cli;

#[derive(Debug)]
pub struct Generator {
    pub len: u8,
    pub upper: bool,
    pub lower: bool,
    pub num: bool,
    pub sym: bool,
    pub generated: String,
}

impl Generator {
    pub fn new(args: Cli) -> Generator {
        // destructure the parsed arguments into variables
        let Cli { 
            length: len, 
            no_upper: mut upper, 
            no_lower: mut lower, 
            no_num: mut num, 
            symbols: sym 
        } = args; 
        let generated = String::from("");

        // flip booleans to match
        upper = !upper;
        lower = !lower;
        num = !num;

        // initialize and return Generator struct
        Generator { len, upper, lower, num, sym, generated }
    }

    pub fn make(&mut self) -> () {
        println!("{:?}", self);
    }
}
