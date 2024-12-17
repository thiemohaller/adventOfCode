use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

type ColorCount = HashMap<String, i32>;
type Draws = Vec<ColorCount>;

struct Game {
    game_number: i32,
    draws: Draws,
}

impl Game {
    // Basic constructor for an empty Game
    fn new() -> Game {
        Game {
            game_number: -1,
            draws: Vec::new(),
        }
    }

    fn parse(game_str: &str) -> Result<Game, &'static str> {
        let parts: Vec<&str> = game_str.splitn(2, ": ").collect();
        if parts.len() != 2 {
            return Err("Invalid game format");
        }

        let game_number = parts[0]
            .split_whitespace()
            .last()
            .ok_or("No game number found")?
            .parse::<i32>()
            .map_err(|_| "Invalid game number")?;

        let draws_str = parts[1];
        let draw_parts: Vec<&str> = draws_str.split("; ").collect();
        let mut draws = Vec::new();

        for draw_str in draw_parts {
            let mut color_count = ColorCount::new();
            for color_pair in draw_str.split(", ") {
                let pair_parts: Vec<&str> = color_pair.split_whitespace().collect();
                if pair_parts.len() != 2 {
                    return Err("Invalid color-count pair");
                }
                let count = pair_parts[0].parse::<i32>().map_err(|_| "Invalid count")?;
                let color = pair_parts[1].to_string();
                color_count.insert(color, count);
            }
            draws.push(color_count);
        }

        Ok(Game { game_number, draws })
    }
}

fn is_game_possible(game: &Game) -> bool {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    for draw in &game.draws {
        let red_drawn = draw.get("red").unwrap_or(&0);
        let green_drawn = draw.get("green").unwrap_or(&0);
        let blue_drawn = draw.get("blue").unwrap_or(&0);

        if red_drawn > &red_limit {
            return false;
        }

        if green_drawn > &green_limit {
            return false;
        }

        if blue_drawn > &blue_limit {
            return false;
        }
        // red_total += draw.get("red").unwrap_or(&0);
        // green_total += draw.get("green").unwrap_or(&0);
        // blue_total += draw.get("blue").unwrap_or(&0);
    }

    true
    // red_total <= red_limit && green_total <= green_limit && blue_total <= blue_limit
}

fn get_power_of_minimum_possible_cubes(game: &Game) -> i32 {
    let mut minimum_red = 0;
    let mut minimum_green = 0;
    let mut minimum_blue = 0;

    for draw in &game.draws {
        let red_drawn = draw.get("red").unwrap_or(&0);
        if red_drawn > &minimum_red {
            minimum_red = *red_drawn;
        }

        let green_drawn = draw.get("green").unwrap_or(&0);
        if green_drawn > &minimum_green {
            minimum_green = *green_drawn;
        }

        let blue_drawn = draw.get("blue").unwrap_or(&0);
        if blue_drawn > &minimum_blue {
            minimum_blue = *blue_drawn;
        }
    }

    let power = minimum_red * minimum_green * minimum_blue;

    println!("Minimum Red: {}", minimum_red);
    println!("Minimum Green: {}", minimum_green);
    println!("Minimum Blue: {}", minimum_blue);
    println!("Power: {}", power);

    power
}

fn main() {
    println!("Hello World!");
    let file_path = "src/input.txt";
    let file = File::open(file_path).expect("Problem opening the file");
    let mut all_games = Vec::new();
    let reader = BufReader::new(file);

    // reading all games from the file
    for line in reader.lines() {
        let line = line.expect("Problem reading line from the file");
        // match to ensure it is either parsed or throws an error
        match Game::parse(&line) {
            Ok(game) => {
                all_games.push(game);
            }
            Err(e) => {
                eprintln!("Error parsing game: {}", e);
            }
        }
    }

    // let mut sum_of_indices_of_possible_games = 0;
    // for game in &all_games {
    //     if is_game_possible(game) {
    //         println!("Game {} is possible", game.game_number);
    //         sum_of_indices_of_possible_games += game.game_number;
    //     } else {
    //         println!("Game {} is not possible", game.game_number);
    //     }
    // }

    let mut sum_of_powers = 0;
    for game in &all_games {
        let power = get_power_of_minimum_possible_cubes(&game);
        sum_of_powers += power;
    }

    // println!("sum: {}", sum_of_indices_of_possible_games);
    println!("sum of power: {}", sum_of_powers);
}
