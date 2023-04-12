// Reverse a string in Rust.

fn revStr(s: &str) -> String {
    let reversed_chars: Vec<char> = s.chars().rev().collect();
    let reversed_string: String = reversed_chars.into_iter().collect();
    reversed_string
}

fn main() {
    let s = "Hello, World!";
    let reversed = revStr(s);
    println!("Original string: {}", s);
    println!("Reversed string: {}", reversed);
}
