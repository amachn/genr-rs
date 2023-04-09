mod generator;
mod io;

use arboard::Clipboard;

use std::{
    env,
    io::{ 
        stdin,
        stdout,
    },
};

use generator::Generator;
use io::*;

fn main() -> () {
    // enable dev backtraces
    env::set_var("RUST_BACKTRACE", "1"); // TODO: remove when out of dev

    // create a new, mutable generator instance
    let mut generator = Generator::new();

    // i/o handles
    let inhnd = stdin();
    let mut outhnd = stdout();

    // clipboard
    let mut clipboard = Clipboard::new().unwrap();

    // main process loop
    loop {
        print_opts(&generator, &mut outhnd);
        match fetch_usize_input(&inhnd) {
            Ok(0) => return, // exit the program :)
            Ok(1) => {
                // TODO: offer a copy to the clipboard
                let pass = generator.make();
                println!("Generated: {}", pass);

                loop {
                    print_with_flush("Copy to clipboard? [Y\\n]: ", &mut outhnd);
                    let breakpoint = fetch_str_input(&inhnd);

                    match breakpoint.to_lowercase().as_str() {
                        "y" => {
                            clipboard.set_text(pass).unwrap();
                            println!("Done!");
                        },
                        "n" => {},
                        _ => {
                            println!("ERROR: Invalid input! Try again please.");
                            continue;
                        },
                    };

                    break;
                }
            },
            Ok(2) => {
                // collect the new length
                print_with_flush("New Length: ", &mut outhnd);
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
                print_with_flush("
This program runs in an infinite loop unless given an exit command.
There are two primary rules:
    1) At least one charset must be enabled (uppercase, lowercase, numbers, symbols).
    2) The length of the password must be greater than or equal to 8, but no greater than 64.
This project uses the rand crate's cryptographically secure StdRng to generate passwords.
", 
                    &mut outhnd
                );
            },
            Ok(_) => println!("ERROR: Invalid input! Try again..."),
            Err(e) => println!("{e:?}"),
        };
    }
}
