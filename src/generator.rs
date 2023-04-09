use rand::{
    distributions::Uniform,
    prelude::{
        Rng,
        RngCore,
        SeedableRng,
        StdRng,
        thread_rng,
    },
};

// A basic password generator written in Rust.
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

    // Indicates if the charsets have been modified.
    pub updated: bool,

    // The current charset (only created/updated on Generator::make()).
    chars: Vec<char>,

    // The PRNG (cryptographically secure) used to generate the passwords.
    rng: StdRng,

    // Sample range for the PRNG.
    range: Uniform<usize>, 
}

impl Generator {
    pub fn new() -> Generator {
        // generate a random seed to be used with StdRng
        let mut seed = [0u8; 32];
        thread_rng().fill_bytes(&mut seed);

        Generator { 
            length: 8, 
            upper: true,
            lower: true, 
            num: true, 
            sym: false, 
            updated: true,
            chars: Vec::with_capacity(77),
            // we use capacity 77 because: lower (26) + upper (26) + num (10) + sym (15) = 77
            rng: StdRng::from_seed(seed), // create an StdRng based on the seed we've generated
            range: Uniform::new(0, 1), // temporary values until self.update() is called
        }
    }

    pub fn make(&mut self) -> String {
        if self.updated { self.update(); } // updates chars

        // generate the password
        let mut pass = String::new();
        for _ in 0..self.length {
            pass.push(self.chars[self.rng.sample(self.range)]);
        }

        pass
    }

    fn update(&mut self) -> () {
        if !self.chars.is_empty() { self.chars = Vec::with_capacity(77); } // reset charsets
        
        // push all selected charsets to self.chars
        if self.upper {
            for c in 'A' as u8 ..= 'Z' as u8 {
                self.chars.push(c as char);
            }
        }

        if self.lower {
            for c in 'a' as u8 ..= 'z' as u8 {
                self.chars.push(c as char);
            }
        }

        if self.num {
            for c in '0' as u8 ..= '9' as u8 {
                self.chars.push(c as char);
            }
        }

        if self.sym {
            self.chars.append(&mut vec![
                '!', '@', '#', '$', '%', '&', '*', '+', '-', '[', ']', '(', ')', '{', '}',
            ]);
        }

        // reset updated status and create proper range
        self.updated = false;
        self.range = Uniform::new(0, self.chars.len());
    }
}
