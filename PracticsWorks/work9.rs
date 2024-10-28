use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let parts: Vec<&str> = s.split(':').collect();
    let hour = parts[0].parse::<i32>().unwrap();
    let minute = parts[1];
    let second = &parts[2][..2];
    let period = &parts[2][2..];

    let new_hour = match period {
        "AM" => {
            if hour == 12 {
                "00".to_string()
            } else {
                format!("{:02}", hour)
            }
        }
        "PM" => {
            if hour == 12 {
                "12".to_string()
            } else {
                format!("{:02}", hour + 12) 
            }
        }
        _ => panic!("Invalid time format"),
    };

    format!("{}:{}:{}", new_hour, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
