use std::fmt;

// A basic password generator written in Rust.
#[derive(Debug)]
pub struct Generator {
    // The length of the password to be generated.
    pub length: u8,

    // Disable uppercase letters.
    pub upper: bool,

    // Disable lowercase letters.
    pub lower: bool,

    // Disable numbers.
    pub num: bool,

    // Enable symbols.
    pub sym: bool,

    // Stores the generated password.
    pub generated: String,
}

impl fmt::Display for Generator { // TODO: remove when out of dev
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generated)
    }
}

impl Generator {
    pub fn new() -> Generator {
        Generator { 
            length: 8, 
            upper: true,
            lower: true, 
            num: true, 
            sym: false, 
            generated: String::new() ,
        }
    }

    pub fn make(&mut self) -> () {
        // TODO: actually generate the password lol
    }
}
