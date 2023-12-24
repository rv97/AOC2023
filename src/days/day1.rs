mod day1_2;

use std::fmt::Debug;
use std::fs::{File, read_to_string};
use std::io;
use std::io::BufRead;
use std::path::Path;
fn main() {
    const FILE_NAME: &str = "src/inputs/day1.txt";

    let mut strings_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(FILE_NAME) {
        for line in lines {
            if let Ok(ip) = line {
                // println!("String {:?}", ip);
                strings_vec.push(ip);
            }
        }
    }

    println!("{:?}", find_calibration_values_sum(strings_vec));

}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_calibration_values_sum(strings_vec: Vec<String>) -> i32 {
    let mut final_sum: i32 = 0;
    for str in strings_vec {
        let mut first_value: i32 = 0;
        let mut last_value: i32 = 0;
        for c in str.chars() {
            if let Some(digit) = c.to_digit(10) {
                first_value = digit as i32;
                break;
            }
        }

        for c in str.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                last_value = digit as i32;
                break;
            }
        }
        // println!("Number: {}", (first_value*10 + last_value));
        final_sum = final_sum + (first_value*10 + last_value);
    }
    final_sum
}