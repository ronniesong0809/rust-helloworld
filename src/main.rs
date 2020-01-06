use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str( & arg).expect("err"));
    }

    let result = gcd(numbers[0], numbers[1]);

    println!("The gcd of {:?} is {}", numbers, result);
}

fn gcd(mut a: u64, mut b: u64) - > u64 {
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