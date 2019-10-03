use std::{io, thread, time};
use termion::clear;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s = s.trim().parse().unwrap();

    eprintln!("Got \"{}\"", s);
    let mut t = 5;
    while t != 0 {
        eprint!("Continue after {} [Ctrl+C to discard]", t);
        thread::sleep(time::Duration::new(1, 0));
        eprint!("\r{}", clear::CurrentLine);
        t -= 1;
    }

    println!("{}", s);
}
