use utils::utils::read_file::readfile_by_lines;

fn main() {
    const FILE_PATH: &str = "src/inputs/day3.txt";
    let strings_vec = readfile_by_lines(FILE_PATH);
    println!("Final Answer: {}",find_gear_ratios(strings_vec));
}

fn find_gear_ratios(strings_vec: Vec<String>) -> u32 {
    let mut final_sum: u32 = 0;
    let mut pattern: Vec<Vec<char>> = vec![];
    for str in strings_vec {
        let line: Vec<char> = str.chars().collect();
        pattern.push(line);
    }
    let mut valid_numbers_str: Vec<String> = vec![];
    for (row_index, row_value) in pattern.iter().enumerate() {
        let mut number_str: String = String::from("");
        let mut flag: bool = false;
        for (col_index, col_value) in row_value.iter().enumerate() {
            let current_value = pattern[row_index][col_index];
            if let Some(_) = col_value.to_digit(10) {
                if row_index == 0 {
                    if row_index+1 < pattern.len() {
                        if col_index == 0 {
                            if col_index+1 < pattern[row_index].len() {
                                if pattern[row_index+1][col_index] != '.' ||  pattern[row_index+1][col_index+1] != '.' {
                                    flag = true;
                                }
                            }
                        }
                        if col_index>0 && col_index+1<pattern[row_index].len() {
                            if pattern[row_index+1][col_index-1] != '.' || pattern[row_index+1][col_index] != '.' || pattern[row_index+1][col_index+1] != '.' {
                                flag = true;
                            }
                        }
                        if col_index == pattern[row_index].len()-1 {
                            if pattern[row_index+1][col_index-1] != '.' || pattern[row_index+1][col_index] != '.' {
                                flag = true;
                            }
                        }
                    }
                    if col_index+1<pattern[row_index].len() && (pattern[row_index][col_index+1] != '.' && !pattern[row_index][col_index+1].is_digit(10)) {
                        flag = true;
                    }
                    if col_index>0 && (pattern[row_index][col_index-1] != '.' && !pattern[row_index][col_index-1].is_digit(10)) {
                        flag = true;
                    }
                }
                else if row_index == pattern.len()-1 {
                    if row_index-1>=0 {
                        if col_index == 0 {
                            if pattern[row_index-1][col_index] != '.' || pattern[row_index-1][col_index+1] != '.' {
                                flag = true;
                            }
                        }
                        else if col_index == pattern[row_index].len()-1 {
                            if pattern[row_index-1][col_index] != '.' || pattern[row_index-1][col_index-1] != '.' {
                                flag = true;
                            }
                        }
                        else {
                            if pattern[row_index-1][col_index]!='.' || pattern[row_index-1][col_index+1]!='.'|| pattern[row_index-1][col_index-1]!='.' {
                                flag = true;
                            }
                        }
                    }
                    if col_index+1<pattern[row_index].len() {
                        if pattern[row_index][col_index+1] != '.' && !pattern[row_index][col_index+1].is_digit(10) {
                            flag = true;
                        }
                    }
                    if col_index-1>=0 && pattern[row_index][col_index-1]!='.' && !pattern[row_index][col_index-1].is_digit(10) {
                        flag = true;
                    }

                }
                else {
                    if col_index == 0 {
                        if row_index-1 >= 0 {
                            if pattern[row_index-1][col_index] != '.' {
                                flag = true;
                            }
                            if col_index+1<pattern[row_index].len() {
                                if pattern[row_index-1][col_index+1] != '.' {
                                    flag = true;
                                }
                            }
                        }
                        if row_index+1<pattern.len() {
                            if pattern[row_index+1][col_index] != '.' {
                                flag = true;
                            }
                            if col_index+1<pattern[row_index].len() {
                                if pattern[row_index+1][col_index+1] != '.' {
                                    flag = true;
                                }
                            }
                        }
                    }
                    else if col_index == pattern[row_index].len()-1 {
                        if row_index-1 >= 0 {
                            if pattern[row_index-1][col_index] != '.' {
                                flag = true;
                            }
                            if col_index-1>=0 {
                                if pattern[row_index-1][col_index-1] != '.' {
                                    flag = true;
                                }
                            }
                        }
                        if row_index+1<pattern.len() {
                            if pattern[row_index+1][col_index] != '.' {
                                flag = true;
                            }
                            if col_index-1>=0 {
                                if pattern[row_index+1][col_index-1] != '.' {
                                    flag = true;
                                }
                            }
                        }
                    }
                    else {
                        if row_index-1 >= 0 {
                            if col_index-1>=0 && col_index+1<pattern[row_index].len() {
                                if pattern[row_index-1][col_index-1] != '.' || pattern[row_index-1][col_index] != '.' || pattern[row_index-1][col_index+1]!= '.' {
                                    flag = true;
                                }
                            }
                        }
                        if row_index+1<pattern.len() {
                            if col_index-1>=0 && col_index+1<pattern[row_index].len() {
                                if pattern[row_index+1][col_index-1] != '.' || pattern[row_index+1][col_index] != '.' || pattern[row_index+1][col_index+1]!= '.' {
                                    flag = true;
                                }
                            }
                        }
                        if col_index-1>=0 {
                            if pattern[row_index][col_index-1] != '.' && !pattern[row_index][col_index-1].is_digit(10) {
                                flag = true;
                            }
                        }
                        if col_index+1<pattern[row_index].len() {
                            if pattern[row_index][col_index+1] != '.' && !pattern[row_index][col_index+1].is_digit(10) {
                                flag = true;
                            }
                        }
                    }
                }
                number_str.push(current_value);
            } else {
                if number_str.len() > 0 {
                    if flag {
                        valid_numbers_str.push(number_str);
                        flag = false;
                    }
                    number_str = String::new();
                }
            }
        }
    }
    for valid_number in valid_numbers_str {
        final_sum+= valid_number.parse::<u32>().unwrap();
    }
    final_sum
}