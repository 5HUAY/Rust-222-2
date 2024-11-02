use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut bird_count = HashMap::new();

    // Count the frequency of each bird type
    for &bird in arr {
        *bird_count.entry(bird).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut bird_type = 0;

    // Find the bird type with the maximum frequency
    for (&type_id, &count) in &bird_count {
        if count > max_count || (count == max_count && type_id < bird_type) {
            max_count = count;
            bird_type = type_id;
        }
    }

    bird_type
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
