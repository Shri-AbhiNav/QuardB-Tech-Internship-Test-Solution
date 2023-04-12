// Check if a number is prime in Rust.

use std::io;

fn isPrime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a no :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u64 = input.trim().parse().expect("Invalid input");

    if isPrime(n) {
        println!("{} is a prime number", n);
    } else {
        println!("{} is not a prime number", n);
    }
}
