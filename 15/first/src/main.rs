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
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();
    let width = grid.len() / height;
    let movements = lines
        .skip(separating_line_index + 1)
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();

    // simulate the robot
    let mut robot = grid.iter().position(|&c| c == '@').unwrap();
    for step in movements {
        print_grid(&grid, width);
        // calculate next step to move
        let new_pos = get_next_step(width, robot, step);
        if new_pos >= grid.len() {
            unreachable!("Getting out of bounds, this should never happen because there should be walls around")
        }
        // if empty, just step into it
        if grid[new_pos] == '.' {
            grid[robot] = '.';
            robot = new_pos;
            grid[robot] = '@';
        // if there is wall, do nothing
        } else if grid[new_pos] == '#' {
            continue;
        // there is crate, must check whether there is free space behind it
        } else if grid[new_pos] == 'O' {
            // find next "non-crate" place
            let mut deciding_field = new_pos;
            while grid[deciding_field] == 'O' {
                deciding_field = get_next_step(width, deciding_field, step);
                if deciding_field >= grid.len() {
                    unreachable!("Get out of grid when stepping over boxes, shit should never happen")
                }
            }
            // if there is wall directly behind, do nothing
            if grid[deciding_field] == '#' {
                continue
            // else move everything
            } else {
                while deciding_field != robot {
                    let prev = get_next_step(width, deciding_field, match step {
                        '^' => 'v',
                        'v' => '^',
                        '<' => '>',
                        '>' => '<',
                        _ => unreachable!(),
                    });
                    grid[deciding_field] = grid[prev];
                    deciding_field = prev;
                }
                grid[robot] = '.';
                robot = new_pos;
            }
        }
    }

    // calculate result
    let mut sum = 0;
    for i in 0..grid.len() {
        if grid[i] == 'O' {
            let row = i / width;
            let col = i % width;
            let score = 100 * row + col;
            sum += score;
        }
    }
    println!("{}", sum);
}

fn get_next_step(width: usize, robot: usize, step: char) -> usize {
    match step {
        '^' => robot.wrapping_sub(width),
        'v' => robot.wrapping_add(width),
        '<' => robot.wrapping_sub(1),
        '>' => robot.wrapping_add(1),
        _ => unreachable!(),
    }
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