// Implement a function that returns the kth smallest element in a given array.

use std::io;

fn SmallElement(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    Some(sorted_arr[k - 1])
}

fn main() {
    println!("Enter the number of elements in the array: ");
    let mut num_elements = String::new();
    io::stdin().read_line(&mut num_elements).expect("Failed to read input");
    let num_elements = num_elements.trim().parse::<usize>().expect("Invalid input");

    let mut arr: Vec<i32> = Vec::new();
    for i in 0..num_elements {
        println!("Enter element #{}: ", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().parse::<i32>().expect("Invalid input");
        arr.push(input);
    }

    println!("Enter the value of k: ");
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read input");
    let k = k.trim().parse::<usize>().expect("Invalid input");

    let kth_smallest = SmallElement(&arr, k);
    match kth_smallest {
        Some(value) => println!("Kth Smallest Element: {}", value),
        None => println!("Invalid value of k"),
    }
}
