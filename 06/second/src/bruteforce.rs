use std::collections::HashSet;

pub fn bruteforce() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let grid_original: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    // guard starting pos
    let (start_row, col) = grid_original
        .iter()
        .enumerate()
        .find(|(_, row)| row.iter().any(|&c| c == '^'))
        .unwrap();
    let start_col = col.iter().position(|&c| c == '^').unwrap();

    let start_alg = |obstacle_row: usize, obstacle_col: usize| -> bool {
        // initialize variables
        let mut current_row = start_row;
        let mut current_col = start_col;
        let mut facing_dir = '^';
        let mut grid = grid_original.clone();
        grid[obstacle_row][obstacle_col] = '#';
        let mut visited: Vec<Vec<HashSet<char>>> =
            vec![vec![HashSet::new(); grid[0].len()]; grid.len()];
        visited[current_row][current_col].insert(facing_dir);

        // track path
        loop {
            let (next_row_try, next_col_try) = match facing_dir {
                '^' => (current_row.checked_add_signed(-1), Some(current_col)),
                'v' => (Some(current_row + 1), Some(current_col)),
                '<' => (Some(current_row), current_col.checked_add_signed(-1)),
                '>' => (Some(current_row), Some(current_col + 1)),
                _ => panic!("Invalid direction"),
            };

            // protect bounds
            if next_row_try.is_none() || next_col_try.is_none() {
                break;
            }
            let next_row = next_row_try.unwrap();
            let next_col = next_col_try.unwrap();
            if next_row >= grid.len() || next_col >= grid[0].len() {
                break;
            }

            // do step
            if grid[next_row][next_col] == '#' {
                facing_dir = match facing_dir {
                    '^' => '>',
                    'v' => '<',
                    '<' => '^',
                    '>' => 'v',
                    _ => panic!("Invalid direction"),
                };
            } else if visited[next_row][next_col].contains(&facing_dir) {
                return true;
            } else {
                current_row = next_row;
                current_col = next_col;
                visited[current_row][current_col].insert(facing_dir);
            }
        }

        return false;
    };

    // start visiting
    let mut sum_obstacle_possibilities = 0;
    for row in 0..grid_original.len() {
        for col in 0..grid_original[0].len() {
            if grid_original[row][col] != '.' {
                continue;
            }
            if start_alg(row, col) {
                sum_obstacle_possibilities += 1;
            }
        }
    }

    println!("{}", sum_obstacle_possibilities);
}
