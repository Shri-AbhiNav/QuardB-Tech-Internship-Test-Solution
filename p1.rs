//Implement a function that checks whether a given string is a palindrome or not.

use std::io;

fn isPalindrome(s: &str) -> bool {
    let s = s.chars().collect::<String>();
    let s_reversed: String = s.chars().rev().collect();
    s == s_reversed
}

fn main() {
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    println!("Input: '{}'", input);
    println!("Is it a palindrome? {}", isPalindrome(input));
}