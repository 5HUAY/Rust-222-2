use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gradingStudents' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */

fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade // No rounding if the grade is less than 38
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5 // Round up to the next multiple of 5
            } else {
                grade // No rounding
            }
        }
    }).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
