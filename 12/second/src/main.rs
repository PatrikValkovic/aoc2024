use std::collections::HashMap;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    // enlarge grid so we get rid of edge-cases
    {
        let mut enlarged_grid: Vec<Vec<char>> = vec![vec![' '; grid[0].len() * 3]; grid.len() * 3];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                for r in i * 3 .. (i + 1) * 3 {
                    for c in j * 3 .. (j + 1) * 3 {
                        enlarged_grid[r][c] = grid[i][j];
                    }
                }
            }
        }
        grid = enlarged_grid;
    }
    let grid = grid;
    let mut covered: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    let mut sum = 0;
    // reuse some variables
    let mut stack = Vec::new();
    let mut outside_list = Vec::new();
    let mut outside_counts = HashMap::new();
    // go through plots
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // if it was already visited, ignore, as it is already accounted for as part of the plot
            if covered[i][j] {
                continue;
            }
            let symbol = grid[i][j];
            let mut area = 0;
            stack.clear();
            outside_list.clear();
            stack.push((i, j));
            // run BFS within the plot
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                let (r, c) = current;
                // we moved out of the current plot, remember it for edge detection
                if r >= grid.len() || c >= grid[r].len() || grid[r][c] != symbol {
                    outside_list.push(current);
                    continue;
                }
                // already visited, ignore
                // must be after detecting perimeter, because the previous plot can be already accounted for
                if covered[r][c] {
                    continue;
                }
                // visit
                covered[r][c] = true;
                area += 1;
                // expand further
                stack.push((r + 1, c));
                stack.push((r.wrapping_sub(1), c));
                stack.push((r, c + 1));
                stack.push((r, c.wrapping_sub(1)));
            }
            // turn outside stack into hashset to know how many times was each node visited
            // this will tell in how many edges the node participate in
            outside_counts.clear();
            for (r, c) in outside_list.iter() {
                outside_counts.entry((*r, *c)).and_modify(|e| *e += 1).or_insert(1);
            }
            let mut sides = 0;
            // detect edges
            for (r, c) in outside_list.iter() {
                // skip if the node was already counted into edges
                if *outside_counts.get(&(*r, *c)).unwrap() == 0 {
                    continue;
                }
                let directions: Vec<(i32,i32)> = vec!(
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                );
                // collect all nodes in given direction as part of single edge
                for (vertical, horizontal) in directions.iter() {
                    let mut index = 1;
                    loop {
                        let r = r.wrapping_add((index * vertical) as usize);
                        let c = c.wrapping_add((index * horizontal) as usize);
                        if !outside_counts.contains_key(&(r, c)) || *outside_counts.get(&(r, c)).unwrap() == 0 {
                            break;
                        }
                        // decrement value marking the node as participating in current edge
                        outside_counts.entry((r, c)).and_modify(|e| *e -= 1);
                        index += 1;
                    }
                }

                // remaining edges this node participates in is its "rank"
                sides += outside_counts.get(&(*r, *c)).unwrap();
                // no more edges can go through this node
                outside_counts.entry((*r, *c)).and_modify(|e| *e = 0);
            }

            // println!("{}: area: {}, sides: {}", symbol, area/9, sides);
            sum += area * sides / 9;
        }
    }

    println!("{}", sum)
}
