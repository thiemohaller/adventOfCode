use std::{fs::File, io::{self, BufReader, BufRead}, ops::Add, collections::HashSet};

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
    !c.is_alphanumeric() && c != '.'
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

fn retrieve_numbers(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> (i32, Vec<(usize, usize)>) {
    let mut number_as_string = String::new();
    let mut indices = Vec::new();

    let mut current_col = col;
    // Move left to find the start of the number
    while current_col > 0 && matrix[row][current_col - 1].is_digit(10) {
        current_col -= 1;
    }

    // Now collect the entire number and its indices
    while current_col < matrix[row].len() && matrix[row][current_col].is_digit(10) {
        number_as_string.push(matrix[row][current_col]);
        indices.push((row, current_col));
        current_col += 1;
    }

    let num = number_as_string.parse::<i32>().unwrap_or(0);
    (num, indices)
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
    let mut processed_indices: HashSet<(usize, usize)> = HashSet::new();
    
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let cell = matrix[i][j];
            if is_symbol(cell) {
                // println!("found asterisk at {}.{}", i, j);
                let mut neighbors = get_digit_neighbors(&matrix, i, j); 
                digit_neighbors.append(&mut neighbors);
            }
        }
    }

    for (row_index, col_index) in digit_neighbors {
        let (num, indices) = retrieve_numbers(&matrix, row_index, col_index);

        if indices.iter().all(|index| processed_indices.insert(*index)) {
            println!("added number {}", num);
            sum += num;
        }
    }

    println!("sum: {}", sum);

}
