use std::fmt::Debug;
use utils::utils::read_file::readfile_by_lines;

fn main() {
    const FILE_NAME: &str = "src/inputs/day1.txt";
    let strings_vec= readfile_by_lines(FILE_NAME);
    println!("{:?}", find_calibration_values_sum(strings_vec));
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