use std::collections::HashSet;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    // create 2D grid
    let mut grid = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    // what are we looking for
    let mut looking_for = HashSet::new();
    looking_for.insert(String::from("MAS"));
    looking_for.insert(String::from("SAM"));

    // search for occurrences
    let mut sum = 0;
    for i in 1..grid.len()-1 {
        for j in 1..grid[i].len()-1 {
            // looking for the middle of X, must be always "A"
            if grid[i][j] != 'A' {
                continue;
            }
            // don't need to check position, because it is already in the i and j variable bounds
            // get diagonal text
            let first_diagonal_chars = vec![
                grid[i - 1][j - 1],
                grid[i][j],
                grid[i + 1][j + 1]
            ];
            let second_diagonal_chars = vec![
                grid[i - 1][j + 1],
                grid[i][j],
                grid[i + 1][j - 1]
            ];
            let first_diagonal = first_diagonal_chars.iter().collect::<String>();
            let second_diagonal = second_diagonal_chars.iter().collect::<String>();
            // check if both diagonals are valid
            if looking_for.contains(&first_diagonal) && looking_for.contains(&second_diagonal) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
