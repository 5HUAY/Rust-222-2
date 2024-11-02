use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;

    // Ensure we don't exceed the bounds
    if m > s.len() as i32 {
        return count; // If m is greater than the length of s, no valid segment exists
    }

    // Iterate over the array, considering each segment of length m
    for i in 0..=(s.len() as i32 - m) {
        let sum: i32 = s[i as usize..(i + m) as usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = birthday(&s, d, m);

    writeln!(&mut fptr, "{}", result).ok();
}
