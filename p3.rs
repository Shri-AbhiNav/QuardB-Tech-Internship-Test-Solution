// Given a string of words, implement a function that returns the shortest word in the string.

use std::io;

fn shortestWord(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut shortest_word = String::new();

    for word in words {
        if shortest_word.is_empty() || word.len() < shortest_word.len() {
            shortest_word = String::from(word);
        }
    }

    shortest_word
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let shortest_word = shortestWord(&input);
    println!("Shortest word: {}", shortest_word);
}
