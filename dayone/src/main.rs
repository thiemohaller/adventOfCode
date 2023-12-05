use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn process_input(input: String) -> String {
    let mut numbers: Vec<char> = Vec::new();

    for c in input.chars() {
        if c.is_digit(10) {
            numbers.push(c);
        }
    }

    let mut concatenated_numbers = String::new();
    if let Some(&first_number) = numbers.first() {
        concatenated_numbers.push(first_number);
    }

    if let Some(&second_number) = numbers.last() {
        concatenated_numbers.push(second_number);
    }

    // println!("{}", concatenated_numbers);

    concatenated_numbers
}

fn main() {
    println!("Hello World!");
    let file_path = "src/input.txt";
    let file = File::open(file_path).expect("Problem opening the file");

    let reader = BufReader::new(file);
    let mut sum_of_numbers = 0;

    for line in reader.lines() {
        let line = line.expect("Problem reading line from the file");
        let processed_content = process_input(line);

        let parsed_number = processed_content.parse::<i32>().unwrap();
        sum_of_numbers += parsed_number;
    }

    println!("sum: {}", sum_of_numbers);
}

