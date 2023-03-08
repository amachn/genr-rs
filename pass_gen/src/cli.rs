use clap::{ArgAction, Parser};

/// A basic password generator written in Rust.
#[derive(Parser, Debug)]
pub struct Cli {
    /// The length of the password to be generated.
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,

    /// Disable uppercase letters
    #[arg(long, visible_alias = "nu", default_value_t = false, action = ArgAction::SetTrue)]
    pub no_upper: bool,

    /// Disable lowercase letters
    #[arg(long, visible_alias = "nl", default_value_t = false, action = ArgAction::SetTrue)]
    pub no_lower: bool,

    /// Disable numbers
    #[arg(long, visible_alias = "nn", default_value_t = false, action = ArgAction::SetTrue)]
    pub no_num: bool,

    /// Enable symbols
    #[arg(short, long, default_value_t = false, action = ArgAction::SetTrue)]
    pub symbols: bool,
}
