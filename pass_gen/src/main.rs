mod generator;

use std::{
    env,
    io::{
        Write, 
        stdin,
        stdout,
    }, 
};

use generator::Generator;

fn print_opts(gen: &Generator) -> () {
    let &Generator { length, upper, lower, num, sym, .. } = gen;
    print!(
        "
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
    match stdout().flush() {
        Ok(_) => {},
        Err(e) => panic!("Write error: {e:?}"),
    };
}

fn fetch_input() -> Result<usize, &'static str> {
    let mut buf = String::new();
    match stdin().read_line(&mut buf) {
        Ok(_) => {},
        Err(e) => eprintln!("Error reading the input: {e:?}"),
    };
    buf.trim()
        .parse::<usize>()
        .map_err(|_| "ERROR: Input must be a number greater than zero! Try again...")
}

fn main() -> () {
    // enable dev backtraces
    env::set_var("RUST_BACKTRACE", "1"); // TODO: remove when out of dev

    // create a new, mutable generator instance
    let mut generator = Generator::new();

    // main process loop
    loop {
        print_opts(&generator);
        match fetch_input() {
            Ok(0) => return, // exit the program :)
            Ok(1) => {
                // TODO: generate a password and output it (offer a copy to clipboard?)
            },
            Ok(2) => {
                // collect the new length
                print!("New Length: ");
                match stdout().flush() {
                    Ok(_) => {},
                    Err(e) => panic!("Write error: {e:?}"),
                };
                let new = fetch_input();

                // validate: if too small or big, deny change. otherwise, make the change.
                match new {
                    Ok(x) if x < 8 || x > 64 => { // too small/big
                        println!("ERROR: Possible lengths range from 8-64 only!");
                    },
                    Ok(x) => { // good length
                        // this won't raise problems as x must be between 8 and 64, u8 is 0-255
                        generator.length = x as u8;
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
                }
            },
            Ok(7) => {
                // TODO: organize a help menu that describes what can and cannot be done
            },
            Ok(_) => println!("ERROR: Invalid input! Try again..."),
            Err(e) => println!("{e:?}"),
        };
    }
}
