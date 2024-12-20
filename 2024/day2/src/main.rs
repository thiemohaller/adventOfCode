use std::{
    fs::File,
    io::{self, BufRead},
};

enum Order {
    Unknown,
    Ascending,
    Descending,
}

fn report_is_safe(line: &str) -> bool {
    let mut trend: Order = Order::Unknown;

    let levels: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for i in 1..levels.len() {
        let previous_number = levels[i - 1];
        let current_number = levels[i];
        if current_number == previous_number {
            return false;
        }

        let number_is_within_three = (previous_number - current_number).abs() <= 3;
        if !number_is_within_three {
            return false;
        }

        let is_ascending = previous_number > current_number;
        let is_descending = previous_number < current_number;

        match trend {
            Order::Unknown => {
                if is_ascending {
                    trend = Order::Ascending;
                } else if is_descending {
                    trend = Order::Descending;
                }
            }
            Order::Ascending => {
                if is_descending {
                    return false;
                }
            }
            Order::Descending => {
                if is_ascending {
                    return false;
                }
            }
        }
    }

    true
}

fn read_levels(path: &str) -> io::Result<i32> {
    // report = line, each entry = level
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // go through each line, check if the report is safe
    // then filter out all the values that returned true and count the amount of them
    let safe_reports = reader
        .lines()
        .map(|line| line.and_then(|l| Ok(report_is_safe(&l))))
        .filter(|res| matches!(res, Ok(true)))
        .count() as i32;

    Ok(safe_reports)
}

fn main() {
    let input = "input.txt";
    let number_of_safe_reports = read_levels(&input);

    println!(
        "Number of safe reports: {:?}",
        number_of_safe_reports.unwrap()
    );
}
