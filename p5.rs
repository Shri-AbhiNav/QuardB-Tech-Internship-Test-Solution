// Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid1 = len / 2 - 1;
        let mid2 = len / 2;
        return (arr[mid1] + arr[mid2]) as f64 / 2.0;
    } else {
        return arr[len/2] as f64;
    }
}

fn main() {
    println!("Enter an array: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input.split(',')
        .map(|s| s.trim().parse().expect("Invalid input"))
        .collect();

    let median_val = median(&arr);
    println!("Median: {:.2}", median_val);
}
