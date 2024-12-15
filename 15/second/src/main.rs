fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let separating_line_index = lines
        .clone()
        .position(|line| line.trim().len() == 0)
        .unwrap();
    let height = separating_line_index;
    let mut grid = lines
        .clone()
        .take(separating_line_index)
        .flat_map(|line| {
            line.chars().flat_map(|char| match char {
                '#' => vec!['#', '#'],
                'O' => vec!['[', ']'],
                '.' => vec!['.', '.'],
                '@' => vec!['@', '.'],
                _ => unreachable!("No other sybol should appear in the input"),
            })
        })
        .collect::<Vec<_>>();
    let width = grid.len() / height;
    let movements = lines
        .skip(separating_line_index + 1)
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();
    let mut belonging = vec![false; grid.len()];

    // simulate the robot
    let mut robot = grid.iter().position(|&c| c == '@').unwrap();
    'steps: for step in movements {
        print_grid(&grid, width);
        // calculate next step to move
        let (horizontal_movement, vertical_movement) = match step {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => unreachable!(),
        };
        let new_pos = (robot as i32 + horizontal_movement + vertical_movement * width as i32) as usize;
        if new_pos >= grid.len() {
            unreachable!("Getting out of bounds, this should never happen because there should be walls around")
        }
        // it is empty space, just go there
        if grid[new_pos] == '.' {
            grid[robot] = '.';
            robot = new_pos;
            grid[robot] = '@';
            continue;
        }
        // it is wall, don't go anywhere
        if grid[new_pos] == '#' {
            continue;
        }
        // calculate horizontal and vertical separately
        if vertical_movement != 0 {
            // moving vertically
            // locations that needs to be expanded
            let mut to_process_locations = Vec::new();
            to_process_locations.push(new_pos);
            belonging.fill(false);
            // expand locations
            while !to_process_locations.is_empty() {
                let to_analyse = to_process_locations.pop().unwrap();
                // some box is blocked by wall, can't move anything
                if grid[to_analyse] == '#' {
                    continue 'steps;
                }
                // empty space, can move there
                if grid[to_analyse] == '.' {
                    continue;
                }
                // box, need to expand further
                let second_half_position = match grid[to_analyse] {
                    '[' => to_analyse + 1,
                    ']' => to_analyse - 1,
                    _ => unreachable!("Only boxes should be here"),
                };
                belonging[to_analyse] = true;
                belonging[second_half_position] = true;
                to_process_locations.push(
                    second_half_position.wrapping_add((vertical_movement * width as i32) as usize)
                );
                to_process_locations.push(
                    to_analyse.wrapping_add((vertical_movement * width as i32) as usize)
                )
            }
            // based on direction, iterate from top or bottom of the grid
            let iterations_base = width..grid.len() - width;
            let iterations = if step == '^' {
                Box::new(iterations_base)
            } else {
                Box::new(iterations_base.rev()) as Box<dyn Iterator<Item = usize>>
            };
            // if the bellow/above tile belongs to covered boxes, move it
            for i in iterations {
                let current = i;
                let prev = current.wrapping_sub((vertical_movement * width as i32) as usize);
                if belonging[prev] {
                    grid[current] = grid[prev];
                    grid[prev] = '.';
                }
            }
            // move robot
            grid[robot] = '.';
            robot = new_pos;
            grid[robot] = '@';
        } else {
            // moving horizontally
            // find first non-box space from robot in given direction
            let mut deciding_field = new_pos;
            while grid[deciding_field] == '[' || grid[deciding_field] == ']' {
                deciding_field = deciding_field.wrapping_add(horizontal_movement as usize);
            }
            // if there is wall directly behind, do nothing
            if grid[deciding_field] == '#' {
                continue;
            }
            // else move everything
            while deciding_field != robot {
                let prev = deciding_field.wrapping_sub(horizontal_movement as usize);
                grid[deciding_field] = grid[prev];
                deciding_field = prev;
            }
            grid[robot] = '.';
            robot = new_pos;
        }
    }
    print_grid(&grid, width);

    // calculate result
    let mut sum = 0;
    for i in 0..grid.len() {
        if grid[i] == '[' {
            let row = i / width;
            let col = i % width;
            let score = 100 * row + col;
            sum += score;
        }
    }
    println!("{}", sum);
}

const SHOULD_PRINT: bool = false;
fn print_grid(grid: &[char], width: usize) {
    if SHOULD_PRINT {
        for (i, &c) in grid.iter().enumerate() {
            print!("{}", c);
            if (i + 1) % width == 0 {
                println!();
            }
        }
        println!()
    }
}
