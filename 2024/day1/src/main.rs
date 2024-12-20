use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

fn part_one(path: &str) -> io::Result<i32> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    // parse each column
    for line in contents.lines() {
        let mut nums = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (nums.next(), nums.next()) {
            if let (Ok(n1), Ok(n2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                col1.push(n1);
                col2.push(n2);
            }
        }
    }

    let mut differences = Vec::new();
    // sort columns to have smalles <-> smallest
    col1.sort();
    col2.sort();

    for (i, j) in col1.iter().zip(col2.iter()) {
        let distance = (i - j).abs();
        differences.push(distance);
    }

    let sum_of_distances: i32 = differences.iter().sum();

    Ok(sum_of_distances)
}

fn part_two(path: &str) -> io::Result<i32> {
    // This time, you'll need to figure out exactly how often each number from the left list appears in the right list.
    // Calculate a total similarity score by adding up each number in the left list after multiplying it by the number
    // of times that number appears in the right list.

    // parse input
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in contents.lines() {
        let mut nums = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (nums.next(), nums.next()) {
            if let (Ok(n1), Ok(n2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                col1.push(n1);
                col2.push(n2);
            }
        }
    }
    
    let mut occurance_dictionary: HashMap<i32, usize> = HashMap::new();
    let mut result_vector: Vec<i32> = Vec::new();
    
    // todo optimize this
    for i in col2.iter() {
        // todo dive a bit deeper on how dereferencing works in rust
        *occurance_dictionary.entry(*i).or_insert(0) += 1;
    }

    for _j in col1.iter() {
        if let Some(&occurances) = occurance_dictionary.get(_j) {
            let current_value = _j * occurances as i32;
            result_vector.push(current_value);
        }
    }

    let similarity_score = result_vector.iter().sum();
    
    return Ok(similarity_score);
}

fn main() {
    let path = "input.txt";
    match part_one(path) {
        Ok(result) => println!("Part One Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    match part_two(path) {
        Ok(result) => println!("Part Two Result: {}", result),
        Err(e) => eprintln!("Error: {}", e)
    }

    println!("So long, and thanks for all the fish! 🐬");
}
