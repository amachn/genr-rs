use clap::{ArgAction, Parser};

use std::{fmt, process::exit};

/// A basic password generator written in Rust.
#[derive(Debug, Parser)]
pub struct Generator {
    /// The length of the password to be generated.
    #[arg(short, long, visible_alias = "len", default_value_t = 8)]
    pub length: u8,

    // the logic for upper/lower/num/sym is a bit goofy, but makes CLI usage simpler for the user

    /// Disable uppercase letters.
    #[arg(long = "no_upper", visible_alias = "nu", default_value_t = true, action = ArgAction::SetFalse)]
    pub upper: bool,

    /// Disable lowercase letters.
    #[arg(long = "no_lower", visible_alias = "nl", default_value_t = true, action = ArgAction::SetFalse)]
    pub lower: bool,

    /// Disable numbers.
    #[arg(long = "no_num", visible_alias = "nn", default_value_t = true, action = ArgAction::SetFalse)]
    pub num: bool,

    /// Enable symbols.
    #[arg(short, long, visible_alias = "symbols", default_value_t = false, action = ArgAction::SetTrue)]
    pub sym: bool,

    // this one is not an argument in the Parser and is exclusively for generation use
    #[clap(skip)]
    pub generated: String,
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generated)
    }
}

impl Generator {
    pub fn new() -> Generator {
        let mut generator = Self::parse(); 
        match Self::validate(&generator) {
            Ok(_) => { 
                generator.generated = String::new();
                generator 
            },
            Err(error) => {
                println!("{}", error);
                exit(1)
            },
        }
    }

    pub fn make(&mut self) -> () {
        
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.upper && self.lower && self.num && self.sym && false { // if all charsets are off
            Err("ERROR: Cannot disable all possible characters!")
        } else if self.length < 4 || self.length > 64 { // if length is <4 or >64
            Err("ERROR: Possible lengths range from 4-64 only!")
        } else { // otherwise, arguments are valid
            Ok(())
        }
    }
}
