mod generator;

use std::{
    env,
    io::{
        Stdin,
        Stdout,
        Write, 
        stdin,
        stdout,
    },
};

use generator::Generator;

fn print_opts(gen: &Generator, outhnd: &mut Stdout) -> () {
    let &Generator { length, upper, lower, num, sym, .. } = gen;

    print!("
Options:
    1) Generate a new password.
    2) Set a new length. [ currently: {length} ]
    3) Toggle uppercase letters. [ currently: {upper} ]
    4) Toggle lowercase letters. [ currently: {lower} ]
    5) Toggle numbers [ currently: {num} ]
    6) Toggle symbols [ currently: {sym} ]
    7) Print a help menu.
    0) Exit.
Select One: "
    );

    match outhnd.flush() {
        Ok(_) => {},
        Err(e) => panic!("Write error: {e:?}"),
    };
}

fn fetch_usize_input(inhnd: &Stdin) -> Result<usize, &'static str> {
    let mut buf = String::new();

    match inhnd.read_line(&mut buf) {
        Ok(_) => {},
        Err(e) => eprintln!("Error reading the input: {e:?}"),
    };

    buf.trim()
        .parse::<usize>()
        .map_err(|_| "ERROR: Input must be a number greater than zero! Try again...")
}

fn fetch_str_input(inhnd: &Stdin) -> String {
    let mut buf = String::new();

    match inhnd.read_line(&mut buf) {
        Ok(_) => {},
        Err(e) => eprintln!("Error reading the input: {e:?}"),
    };

    buf.trim().to_owned()
}

fn main() -> () {
    // enable dev backtraces
    env::set_var("RUST_BACKTRACE", "1"); // TODO: remove when out of dev

    // create a new, mutable generator instance
    let mut generator = Generator::new();

    // i/o handles
    let inhnd = stdin();
    let mut outhnd = stdout();

    // main process loop
    loop {
        print_opts(&generator, &mut outhnd);
        match fetch_usize_input(&inhnd) {
            Ok(0) => return, // exit the program :)
            Ok(1) => {
                // TODO: offer a copy to the clipboard
                let pass = generator.make();
                println!("Generated: {}", pass);
            },
            Ok(2) => {
                // collect the new length
                print!("New Length: ");
                match outhnd.flush() {
                    Ok(_) => {},
                    Err(e) => panic!("Write error: {e:?}"),
                };
                let new = fetch_usize_input(&inhnd);

                // validate: if too small or big, deny change. otherwise, make the change.
                match new {
                    Ok(x) if x < 8 || x > 64 => { // too small/big
                        println!("ERROR: Possible lengths range from 8-64 only!");
                    },
                    Ok(x) => { // good length
                        // this won't raise problems as x must be between 8 and 64, u8 is 0-255
                        generator.length = x as u8;
                        generator.updated = true;
                    },
                    Err(e) => println!("{e:?}"),
                }
            },
            Ok(x) if [3, 4, 5, 6].contains(&x) => { // flip the boolean and verify
                let flip = |num: usize, generator: &mut Generator| -> () {
                    match num {
                        3 => generator.upper = !generator.upper,
                        4 => generator.lower = !generator.lower,
                        5 => generator.num = !generator.num,
                        6 => generator.sym = !generator.sym,
                        _ => unreachable!(),
                    };
                };

                flip(x, &mut generator);

                let charsets: [bool; 4] = [generator.upper, generator.lower, generator.num, generator.sym];

                if charsets.iter().all(|&x| x == false) {
                    flip(x, &mut generator);
                    println!("ERROR: Cannot disable all possible charsets!");
                } else {
                    generator.updated = true;
                }
            },
            Ok(7) => {
                print!("
This program runs in an infinite loop unless given an exit command.
There are two primary rules:
    1) At least one charset must be enabled (uppercase, lowercase, numbers, symbols).
    2) The length of the password must be greater than or equal to 8, but no greater than 64.
This project uses the rand crate's cryptographically secure StdRng to generate passwords.
"
                );

                match outhnd.flush() {
                    Ok(_) => {},
                    Err(e) => panic!("Write error: {e:?}"),
                };
            },
            Ok(_) => println!("ERROR: Invalid input! Try again..."),
            Err(e) => println!("{e:?}"),
        };
    }
}
