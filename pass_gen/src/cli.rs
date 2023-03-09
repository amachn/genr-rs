use clap::{ArgAction, Parser};

/// A basic password generator written in Rust.
#[derive(Parser, Debug)]
pub struct Cli {
    /// The length of the password to be generated.
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,

    /// Disable uppercase letters.
    #[arg(long, visible_alias = "nu", default_value_t = false, action = ArgAction::SetTrue)]
    pub no_upper: bool,

    /// Disable lowercase letters.
    #[arg(long, visible_alias = "nl", default_value_t = false, action = ArgAction::SetTrue)]
    pub no_lower: bool,

    /// Disable numbers.
    #[arg(long, visible_alias = "nn", default_value_t = false, action = ArgAction::SetTrue)]
    pub no_num: bool,

    /// Enable symbols.
    #[arg(short, long, default_value_t = false, action = ArgAction::SetTrue)]
    pub symbols: bool,
}

impl Cli {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.no_upper && self.no_lower && self.no_num && !self.symbols {
            Err("ERROR: Cannot disable all possible characters!")
        } else if self.length < 4 || self.length > 64 {
            Err("ERROR: Possible lengths range from 4-64 only!")
        } else { 
            Ok(()) 
        }         
    }
}
