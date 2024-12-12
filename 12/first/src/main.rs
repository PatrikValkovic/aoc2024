fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut covered: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    // go through plots
    let mut sum = 0;
    let mut stack = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // only if not covered -> already counted s part of different plot
            if covered[i][j] {
                continue;
            }
            let symbol = grid[i][j];
            let mut area = 0;
            let mut perimeter = 0;
            stack.clear();
            stack.push((i,j));

            // run BFS within the plot
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                let (r, c) = current;
                // getting out of plot, count into perimeter
                if r >= grid.len() || c >= grid[r].len() || grid[r][c] != symbol {
                    perimeter += 1;
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
                stack.push((r+1, c));
                stack.push((r.wrapping_sub(1), c));
                stack.push((r, c+1));
                stack.push((r, c.wrapping_sub(1)));
            }
            // println!("For symbol {}: area={} perimeter={}, {:?}", symbol, area, perimeter, outside_stack);
            sum += area * perimeter;
        }
    }

    println!("{}", sum)
}
