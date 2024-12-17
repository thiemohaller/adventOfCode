use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::ops::Add;

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

// &str is a string slice, representing a view into a string. 
// It's generally used when you want to borrow a string without taking ownership of it. 
// This is efficient because it doesn't require allocating memory or copying the string data.
// String means I own the object -> memory allocation
fn replace_strings_with_numbers(input: String) -> String {
    println!("string {}", input);

    let mut result = input;
    result = result.replace("zero", "zero0zero");
    result = result.replace("one", "one1one");
    result = result.replace("two", "two2two");
    result = result.replace("three", "three3three");
    result = result.replace("four", "four4four");
    result = result.replace("five", "five5five");
    result = result.replace("six", "six6six");
    result = result.replace("seven", "seven7seven");
    result = result.replace("eight", "eight8eight");
    result = result.replace("nine", "nine9nine");

    println!("replaced string {}", result);

    result
}

fn main() {
    println!("Hello World!");
    let file_path = "src/input.txt";
    let file = File::open(file_path).expect("Problem opening the file");

    let reader = BufReader::new(file);
    let mut sum_of_numbers = 0;

    for line in reader.lines(){
            let line = line.expect("Problem reading line from the file");
            let replaced_input = replace_strings_with_numbers(line);
            let processed_content = process_input(replaced_input);

            let parsed_number = processed_content.parse::<i32>().unwrap();
            sum_of_numbers += parsed_number;
    }

    println!("sum: {}", sum_of_numbers);
}

