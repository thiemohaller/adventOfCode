use std::{fs::File, io::{self, BufReader, BufRead}, ops::Add};

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1), // Top Left
    (-1, 0),  // Top
    (-1, 1),  // Top Right
    (0, -1),  // Left
    (0, 1),   // Right
    (1, -1),  // Bottom Left
    (1, 0),   // Bottom
    (1, 1),   // Bottom Right
];

fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() || c == '.'
}

fn get_digit_neighbors(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut digit_neighbors = Vec::new();

    for &(row_offset, col_offset) in NEIGHBOR_OFFSETS.iter() {
        let new_row = row as isize + row_offset;
        let new_col = col as isize + col_offset;

        if new_row >= 0 && new_row < matrix.len() as isize && new_col >= 0 && new_col < matrix[new_row as usize].len() as isize {
            let neighbor = matrix[new_row as usize][new_col as usize];
            if neighbor.is_digit(10) {
                digit_neighbors.push((new_row as usize, new_col as usize));
            }
        }
    }

    digit_neighbors
}

// start at row,col, then move left until either end or non-digit is read, prepend to string
// repeat for righthandside with append
fn retrieve_numbers(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut number_as_string = String::new();
    
    // add the digit at the current index to the string
    let current_entry = matrix[row][col];
    if current_entry.is_digit(10) {
        number_as_string.push(current_entry);
        println!("Starting with current entry: {} at index {}.{}", number_as_string, row, col); // Debug print
    }

    // Go left from the initial position and prepend digits to the string
    let mut current_col = col;
    while current_col > 0 {
        // Move to the left
        current_col -= 1; 
        let entry = matrix[row][current_col];
        // println!("Checking left: {}", entry); // Debug print
        
        if entry.is_digit(10) {
            number_as_string = entry.to_string() + &number_as_string;
            // println!("Current number (left): {}", number_as_string); // Debug print
        } else {
            // we have reached the left end of the string
            break;
        }
    }

    // Reset current_col to the initial position 
    // in order to repeat the whole thing for the right side of the string
    current_col = col;

    // Go right from the initial position and append digits to the string
    while current_col + 1 < matrix[row].len() {
        // Move to the right
        current_col += 1;
        let entry = matrix[row][current_col];
        // println!("Checking right: {}", entry); // Debug print

        if entry.is_digit(10) {
            number_as_string.push(entry);
        //  println!("Current number (right): {}", number_as_string); // Debug print
        } else {
            // we have reached the right end of the string
            break;
        }
    }

    println!("Final number string: {}", number_as_string); // Debug print

    // Parse the string to an integer
    match number_as_string.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing number: {}", e); // Error message if parsing fails
            0 // Return a default value in case of error
        }
    }
}

fn read_and_parse_schematics() -> Vec<Vec<char>> {
    let file_path = "src/input.txt";    
    let file = File::open(file_path).expect("Problem opening the file");
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Problem reading line from the file");
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    matrix
}

fn main() {
    println!("Hello, world!");
    let matrix: Vec<Vec<char>> = read_and_parse_schematics();
    let mut digit_neighbors = Vec::new();
    let mut sum = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let cell = matrix[i][j];
            if is_symbol(cell) {
                println!("found asterisk at {}.{}", i, j);
                let mut neighbors = get_digit_neighbors(&matrix, i, j); 
                digit_neighbors.append(&mut neighbors);
            }
        }
    }

    print!("Digit indices surrounding: ");
    for (row_index, col_index) in &digit_neighbors {
        print!("({}, {}), ", row_index, col_index);
    }

    println!();
    for i in digit_neighbors {
        // in our vector<usize,usize> 0 is the row and 1 is the col
        sum += retrieve_numbers(&matrix, i.0, i.1);
    }

    println!("sum: {}", sum);

}
