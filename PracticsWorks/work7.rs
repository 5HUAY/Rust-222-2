use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let sum: i32 = arr.iter().sum();
    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    let min_sum = sum - max_value;
    let max_sum = sum - min_value;

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
