//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

use std::io;

fn firstOccurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len()-1;

    while left <= right {
        let mid = left + (right - left)/2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid+1;
        } else {
            right = mid-1;
        }
    }

    None
}

fn main() {
    println!("Enter sorted array: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Failed to parse integer"))
        .collect();

    println!("Enter target: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let target: i32 = input.trim().parse().expect("Failed to parse target integer");

    if let Some(index) = firstOccurrence(&arr, target) {
        println!("Target {} found at index {}", target, index);
    } else {
        println!("Target {} not found in the array", target);
    }
}
