use utils::utils::read_file::readfile_by_lines;
fn main() {
    const FILE_NAME: &str = "src/inputs/day2.txt";
    let strings_vec: Vec<String> = readfile_by_lines(FILE_NAME);
    println!("Game Total: {}", find_games_that_match(strings_vec));
}

fn find_games_that_match(strings_vec: Vec<String>) -> i32{
    let mut game_total: i32 = 0;
    for game in strings_vec {
        let parts: Vec<&str> = game.split(":").collect();
        let mut current_game_number: i32 = 0;
        if let Some(game_str) = parts[0].trim().split_whitespace().last() {
            if let Ok(game_number) = game_str.parse::<i32>() {
                current_game_number = game_number;
            }
        }
        let trials: Vec<&str> = parts[1].split(";").collect();
        let mut flag: bool = true;
        for trial in trials {
            let tokens: Vec<&str> = trial.split_whitespace().collect();
            let mut current_value = 0;
            for (index, value) in tokens.iter().enumerate() {
                if let Ok(count) = value.parse::<i32>() {
                    current_value = count;
                    continue;
                }
                match value {
                    &"red" | &"red," => {
                        if current_value > 12 {
                            flag = false;
                            break;
                        }
                        current_value = 0;
                    },
                    &"blue" | &"blue," => {
                        if current_value > 14 {
                            flag = false;
                            break;
                        }
                        current_value = 0;
                    }
                    &"green" | &"green," => {
                        if current_value > 13 {
                            flag = false;
                            break;
                        }
                        current_value = 0;
                    }
                    _ => panic!("Wrong text"),
                }
            }
        }
        if flag == true {
            game_total += current_game_number;
        }
    }
    game_total
}