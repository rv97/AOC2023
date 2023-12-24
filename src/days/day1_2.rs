use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::collections::BTreeMap;
use utils::utils::read_file::readfile_by_lines;

fn main() {
    const FILE_NAME: &str = "src/inputs/day1_2.txt";
    let mut strings_vec= readfile_by_lines(FILE_NAME);
    println!("{:?}", find_calibration_values_sum(strings_vec));
}

fn find_calibration_values_sum(strings_vec: Vec<String>) -> i32 {
    let mut final_sum: i32 = 0;
    for str in strings_vec {
        let mut num1 = 0;
        let mut num2 = 0;
        let mut all_indices: Vec<Vec<i32>> = vec![];
        let numbers_indices = find_first_and_last_numbers(str.clone());
        all_indices.extend(numbers_indices);
        if let Ok(text_indices) = find_first_and_last_text_numbers(str) {
            all_indices.extend(text_indices);
        }
        if all_indices.len() > 2 {
            if all_indices[0].len() > 0 && all_indices[1].len() > 0 {
                if all_indices[0][0] > all_indices[2][0] {
                    num1 = all_indices[2][1];
                } else {
                    num1 = all_indices[0][1];
                }
                if all_indices[1][0] > all_indices[3][0] {
                    num2 = all_indices[1][1];
                } else {
                    num2 = all_indices[3][1];
                }
            } else {
                num1 = all_indices[2][1];
                num2 = all_indices[3][1];
            }
        } else {
            num1 = all_indices[0][1];
            num2 = all_indices[1][1];
        }
        // println!("Number is: {}", (num1*10 + num2));
        final_sum = final_sum + (num1*10 + num2);
    }
    final_sum
}

fn find_first_and_last_numbers(string_input: String) -> Vec<Vec<i32>> {
    let mut first_index: Vec<i32> = vec![];
    let mut last_index: Vec<i32> = vec![];
    for (ind, c) in string_input.char_indices() {
        if let Some(digit) = c.to_digit(10) {
            first_index.push(ind as i32);
            first_index.push(digit as i32);
            break;
        }
    }

    for (ind, c) in string_input.char_indices().rev() {
        if let Some(digit) = c.to_digit(10) {
            last_index.push(ind as i32);
            last_index.push(digit as i32);
            break;
        }
    }
    vec![first_index, last_index]
}

fn find_first_and_last_text_numbers(string_input: String) -> Result<Vec<Vec<i32>>, String> {
    let numbers_vec = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut all_indexes: BTreeMap<i32, i32> = BTreeMap::new();
    for (ind, number) in numbers_vec.iter().enumerate() {
        // let matching_indexes: Vec<usize> = string_input.matches(number).map(|m| m.trim_start()).collect();
        // for index in matching_indexes {
        //     all_indexes.insert(index as i32, (ind+1) as i32);
        // }
        // if let Some(index) = string_input.find(number) {
        //     all_indexes.insert(index as i32, (ind+1) as i32);
        // }

        let occurences: Vec<_> = string_input.match_indices(number).collect();
        for (o1, _) in occurences {
            all_indexes.insert(o1 as i32, (ind+1) as i32);
        }

        // let mut occurences = Vec::new();
        // let mut index = 0;
        // while let Some(i) = string_input[index..].find(number) {
        //     index += i+1;
        //     occurences.push(index as i32 - 1);
        // }
        //
        // if !occurences.is_empty() {
        //     for occurrence in occurences {
        //         all_indexes.insert(occurrence, (ind+1) as i32);
        //     }
        // }
    }
    if all_indexes.len() > 0 {
        let mut result: Vec<Vec<i32>> = vec![];
        if let Some((f1, f2)) = all_indexes.first_key_value() {
            result.push(vec![*f1, *f2]);
        } else {
            return Err("No text indexes found".to_string());
        }
        if let Some((f1, f2)) = all_indexes.last_key_value() {
            result.push(vec![*f1, *f2]);
        } else {
            return Err("No text indexes found".to_string());
        }
        Ok(result)
    } else {
        Err("No text number found".to_string())
    }
    // Ok(vec![])
}