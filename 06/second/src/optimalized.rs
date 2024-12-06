pub fn optimalized() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut start_row = usize::MAX;
    let mut start_col = usize::MAX;
    let mut grid_original: Vec<char> = Vec::with_capacity(height * width);
    for (row, line) in lines.iter().enumerate() {
        let chars = line.chars();
        for (col, c) in chars.enumerate() {
            grid_original.push(c);
            if c == '^' {
                start_row = row;
                start_col = col;
            }
        }
    }
    let grid_original = grid_original;
    let start_row = start_row;
    let start_col = start_col;
    let mut grid = grid_original.clone();
    let mut visited = vec![0u8; height * width];

    let dir_to_index = |dir: char| -> u8 {
        match dir {
            '^' => 0x1,
            'v' => 0x2,
            '<' => 0x4,
            '>' => 0x8,
            _ => panic!("Invalid direction"),
        }
    };

    // the path simulation itself
    let run_simulation = |grid: &Vec<char>, visited: &mut Vec<u8>| -> bool {
        visited.fill(0);

        let mut current_row = start_row;
        let mut current_col = start_col;
        let mut facing_dir = '^';
        visited[current_row * width + current_col] |= dir_to_index(facing_dir);

        loop {
            let (next_row_try, next_col_try) = match facing_dir {
                '^' => (current_row.checked_sub(1), Some(current_col)),
                'v' => (Some(current_row + 1), Some(current_col)),
                '<' => (Some(current_row), current_col.checked_sub(1)),
                '>' => (Some(current_row), Some(current_col + 1)),
                _ => panic!("Invalid direction"),
            };

            if next_row_try.is_none() || next_col_try.is_none() {
                break;
            }
            let next_row = next_row_try.unwrap();
            let next_col = next_col_try.unwrap();
            if next_row >= height || next_col >= width {
                break;
            }

            let next_index = next_row * width + next_col;
            if grid[next_index] == '#' {
                facing_dir = match facing_dir {
                    '^' => '>',
                    'v' => '<',
                    '<' => '^',
                    '>' => 'v',
                    _ => panic!("Invalid direction"),
                };
            } else if visited[next_index] & dir_to_index(facing_dir) > 0 {
                return true;
            } else {
                current_row = next_row;
                current_col = next_col;
                visited[current_row * width + current_col] |= dir_to_index(facing_dir);
            }
        } 
        return false;
    };
    run_simulation(&grid, &mut visited);
    let visited_indexes = (0..height*width)
        .filter(|index| visited[*index] > 0)
        .collect::<Vec<usize>>();

    // try to place obstacles on the path
    let mut sum_obstacle_possibilities = 0;
    for index in visited_indexes {
        grid[index] = '#';
        if run_simulation(&grid, &mut visited) {
            sum_obstacle_possibilities += 1;
        }
        grid[index] = '.';
    }

    println!("{}", sum_obstacle_possibilities);
}
