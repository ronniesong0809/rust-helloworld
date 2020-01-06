use std::env;
use std::str::FromStr;
use std::io::Write;
use std::io;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str( & arg).expect("err parsing"));
    }

    if numbers.len() == 0 {
        writeln!(io::stderr(), "Usage: cargo run [number] [number] ...").unwrap();
        std::process::exit(1);
    }

    let mut a = numbers[0];
    for b in &numbers[1..] {
        a = gcd(a, *b);
    }

    println!("The gcd of {:?} is {}", numbers, a);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            let temp = b;
            b = a;
            a = temp;
        }
        b = b % a;
    }
    a
}