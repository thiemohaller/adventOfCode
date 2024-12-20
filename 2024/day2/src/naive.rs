use std::{fs::File, io::{self, BufRead}
};

enum Order {
    Unknown,
    Ascending,
    Descending
}

fn report_is_safe(line: &str) -> bool {
    let mut trend: Option<Order> = Some(Order::Unknown);
    let levels: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

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
            Some(Order::Unknown) => {
            if is_ascending {
                trend = Some(Order::Ascending);
            } else if is_descending {
                trend = Some(Order::Descending);
            }},
            Some(Order::Ascending) => {
                if is_descending {
                    return false;
                }
            }
            Some(Order::Descending) => {
                if is_ascending {
                    return false;
                }
            }
            None => {
                if is_ascending {
                    trend = Some(Order::Ascending);
                } else if is_descending {
                    trend = Some(Order::Descending);
                }
            }
        }
    }

    return true;
}

fn read_levels(path: &str) -> io::Result<i32> {
    // report = line, each entry = level
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;

    for line in reader.lines() {
        let line = line?;
        if report_is_safe(&line) {
            safe_reports += 1;
        }
    }

    return Ok(safe_reports);
}

fn main() {
    let input = "input.txt";
    let number_of_safe_reports = read_levels(&input);

    println!("Number of safe reports: {:?}", number_of_safe_reports.unwrap());
}
