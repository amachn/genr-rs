use std::io::{
    Stdin,
    Stdout,
    Write,
};

use super::generator::Generator;

pub fn print_with_flush(str: &str, outhnd: &mut Stdout) -> () {
    print!("{}", str);

    match outhnd.flush() {
        Ok(_) => {},
        Err(e) => eprintln!("Write error: {e:?}"),
    };
}

pub fn print_opts(gen: &Generator, outhnd: &mut Stdout) -> () {
    let &Generator { length, upper, lower, num, sym, .. } = gen;

    let formatted = format!("
Options:
    1) Generate a new password.
    2) Set a new length. [ currently: {length} ]
    3) Toggle uppercase letters. [ currently: {upper} ]
    4) Toggle lowercase letters. [ currently: {lower} ]
    5) Toggle numbers. [ currently: {num} ]
    6) Toggle symbols. [ currently: {sym} ]
    7) Print a help menu.
    0) Exit.
Select One: ");

    print_with_flush(&formatted, outhnd);
}

pub fn fetch_usize_input(inhnd: &Stdin) -> Result<usize, &str> {
    let mut buf = String::new();

    match inhnd.read_line(&mut buf) {
        Ok(_) => {},
        Err(e) => eprintln!("Read error: {e:?}"),
    };

    buf.trim()
       .parse::<usize>()
       .map_err(|_| "ERROR: Input must be a number greater than zero! Try again please.")
}

pub fn fetch_str_input(inhnd: &Stdin) -> String {
    let mut buf = String::new();

    match inhnd.read_line(&mut buf) {
        Ok(_) => {},
        Err(e) => eprintln!("Read error: {e:?}"),
    };

    buf.trim().to_owned()
}
