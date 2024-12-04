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

    // search for XMAS
    let mut sum = 0;
    let xmas = ['X', 'M', 'A', 'S'];
    let search_direction: Vec<(i32, i32)> =   vec![
        (-1, -1),
        (-1,  0),
        (-1,  1),
        ( 0, -1),
        ( 0,  1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != 'X' {
                continue;
            }
            for (direction_i, direction_j) in &search_direction {
                // check if we are within grid in given direction
                let max_i = i as i32 + *direction_i * (xmas.len() as i32 - 1);
                let max_j = j as i32 + *direction_j * (xmas.len() as i32 - 1);
                if max_i < 0 || max_i >= grid.len() as i32 || max_j < 0 || max_j >= grid[i].len() as i32 {
                    continue;
                }
                // search for substring
                let mut found = true;
                for k in 1..xmas.len() {
                    let pos_i = (i as i32 + *direction_i * k as i32) as usize;
                    let pos_j = (j as i32 + *direction_j * k as i32) as usize;
                    let expected_char = xmas[k];
                    found = found && grid[pos_i][pos_j] == expected_char;
                    if !found {
                        break;
                    }
                }
                if found {
                    sum += 1;
                }
            }
        }
    }

    // print result
    println!("{}", sum);
}
