fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    // find starting pos
    let (start_row, col) = grid
        .iter()
        .enumerate()
        .find(|(_, row)| row.iter().any(|&c| c == '^'))
        .unwrap();
    let start_col = col.iter().position(|&c| c == '^').unwrap();

    // initialize variables
    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut facing_dir = '^';
    visited[current_row][current_col] = true;

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
        } else {
            current_row = next_row;
            current_col = next_col;
            visited[current_row][current_col] = true;
        }
    }

    // show number of visited pages
    let num_visited = visited
        .iter()
        .map(|row| row.iter().filter(|&&v| v).count())
        .sum::<usize>();

    println!("{}", num_visited);
}
