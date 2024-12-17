use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers_i_have: Vec<i32>,
}

fn parse_input(file_path: &str) -> HashMap<i32, Card> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let mut cards = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let id: i32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();

        let numbers: Vec<&str> = parts[1].split("|").collect();
        let winning_numbers: Vec<i32> = numbers[0]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers_i_have: Vec<i32> = numbers[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        cards.insert(
            id - 1,
            Card {
                id,
                winning_numbers,
                numbers_i_have,
            },
        );
    }

    cards
}

fn compute_points_for_card(card: &Card) -> i32 {
    let mut points = 0;
    for number in &card.numbers_i_have {
        if card.winning_numbers.contains(number) {
            if points == 0 {
                points += 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}

fn check_numbers_for_card(card: &Card) -> i32 {
    let mut matches = 0;
    for number in &card.numbers_i_have {
        if card.winning_numbers.contains(number) {
            matches += 1;
        }
    }

    println!("matches for card {}: {}", card.id, matches);
    matches
}

fn main() {
    println!("Hello, world!");
    let file_path = "src/input.txt";
    let cards = parse_input(file_path);
    let mut final_cards = std::collections::HashMap::new();
    let mut total_cards = 0;

    let first_card = cards.get(&1).unwrap().clone();
    final_cards.insert(1, first_card);
    println!("card 1 added now length {}", final_cards.len());

    for i in 0..final_cards.len() as i32 {
        println!("checking card {}", i);
        let entry = cards.get(&i).unwrap();

        let winning_numbers = check_numbers_for_card(entry);

        if winning_numbers == 0 {
            continue;
        }

        for j in 0..winning_numbers {
            let offset = i + j;
            let new_card = cards.get(&offset).unwrap().clone();
            final_cards.insert(offset,new_card);
            total_cards += 1;
            println!("card {} added", offset);
        }

        println!("length of final cards: {}", final_cards.len());
    }

    println!("amount of final cards: {}", total_cards);
}
