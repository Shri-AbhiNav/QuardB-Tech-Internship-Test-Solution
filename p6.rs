// Implement a function that finds the longest common prefix of a given set of strings.

use std::io;

fn lngCommonPrefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut common_prefix = String::new();

    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if string.len() <= i || string.chars().nth(i) != Some(c) {
                break 'outer;
            }
        }
        common_prefix.push(c);
    }

    common_prefix
}

fn main() {
    println!("Enter the number of strings: ");
    let mut num_strings = String::new();
    io::stdin().read_line(&mut num_strings).expect("Failed to read input");
    let num_strings = num_strings.trim().parse::<usize>().expect("Invalid input");

    let mut strings: Vec<String> = Vec::new();
    for i in 0..num_strings {
        println!("Enter string #{}: ", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_string();
        strings.push(input);
    }

    let common_prefix = lngCommonPrefix(&strings);
    println!("Longest Common Prefix: {}", common_prefix);
}
