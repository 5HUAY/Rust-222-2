use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pageCount(n: i32, p: i32) -> i32 {
    // Turns needed from the front
    let from_front = p / 2;
    
    // Turns needed from the back
    let from_back = if n % 2 == 0 {
        (n / 2) - (p / 2)
    } else {
        (n + 1) / 2 - (p / 2)
    };

    // Return the minimum of both
    from_front.min(from_back)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}
