use utils::utils::read_file::readfile_by_lines;

struct Bag {
    red_cubes: i32,
    blue_cubes: i32,
    green_cubes: i32,
}

impl Bag {
    fn new() -> Self {
        Bag {
            red_cubes: 0,
            blue_cubes: 0,
            green_cubes: 0,
        }
    }

    fn find_power(self) -> i32 {
        self.blue_cubes * self.green_cubes * self.red_cubes
    }
}
fn main() {
    const FILE_PATH: &str = "src/inputs/day2_2.txt";
    let strings_vec = readfile_by_lines(FILE_PATH);
    println!("Total Power: {}", find_games_minimum_total(strings_vec));
}

fn find_games_minimum_total(strings_vec: Vec<String>) -> i32 {
    let mut total_power: i32 = 0;
    for game in strings_vec {
        let parts: Vec<&str> = game.split(":").collect();
        let trials: Vec<&str> = parts[1].split(";").collect();
        let mut current_min_bag_value = Bag::new();
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
                        if current_min_bag_value.red_cubes < current_value {
                            current_min_bag_value.red_cubes = current_value;
                        }
                        current_value = 0;
                    },
                    &"blue" | &"blue," => {
                        if current_min_bag_value.blue_cubes < current_value {
                            current_min_bag_value.blue_cubes = current_value;
                        }
                        current_value = 0;
                    }
                    &"green" | &"green," => {
                        if current_min_bag_value.green_cubes < current_value {
                            current_min_bag_value.green_cubes = current_value;
                        }
                        current_value = 0;
                    }
                    _ => panic!("Wrong text"),
                }
            }
        }
        total_power += current_min_bag_value.find_power();
    }
    total_power
}