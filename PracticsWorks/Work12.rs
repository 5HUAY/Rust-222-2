use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return String::from("YES");
        } else {
            return String::from("NO");
        }
    }
    
    // We can rearrange the conditions
    let (d1, d2) = (x2 - x1, v1 - v2);
    
    if d1 % d2 == 0 && d1 / d2 >= 0 {
        return String::from("YES");
    }
    
    String::from("NO")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();
    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}
