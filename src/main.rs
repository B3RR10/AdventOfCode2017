use std::io::{self, Read};

mod day1;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => (),
    }

    let (sum1, sum2) = day1::day1(buffer);
    println!("Sum1: {}\nSum2: {}", sum1, sum2);
}
