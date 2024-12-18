use std::collections::{BinaryHeap, HashSet};

const GRID_SIZE: usize = 71 + 2;
const RECORDS_TO_TAKE: usize = 1024;

fn main() {
    // read input.txt
    let mut grid = vec!['.'; GRID_SIZE * GRID_SIZE];
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut lines = input.lines();
    for _ in 0..RECORDS_TO_TAKE {
        let line = lines.next().unwrap();
        let mut iter = line.split(",");
        let x = iter.next().unwrap().parse::<usize>().unwrap() + 1;
        let y = iter.next().unwrap().parse::<usize>().unwrap() + 1;
        grid[y * GRID_SIZE + x] = '#';
    }
    for i in 0..GRID_SIZE {
        grid[i * GRID_SIZE] = '#';
        grid[i * GRID_SIZE + GRID_SIZE - 1] = '#';
        grid[i] = '#';
        grid[(GRID_SIZE - 1) * GRID_SIZE + i] = '#';
    }

    // find start and end
    let start = GRID_SIZE + 1;
    let end = GRID_SIZE * GRID_SIZE - GRID_SIZE - 2;

    let (distance, parents) = run_search(&grid, start, end);

    // first part
    let score = distance[end];
    println!("Distance: {}", score);

    let mut path_members = vec![false; grid.len()];
    rebuild_path(start, end, parents, &mut path_members);

    loop {
        let next_byte_pos_str = lines.next().unwrap();
        let next_byte_pos = next_byte_pos_str
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let next_byte_index = (next_byte_pos[1] + 1) * GRID_SIZE + (next_byte_pos[0] + 1);
        grid[next_byte_index] = '#';

        if path_members[next_byte_index] {
            let (distance, parents) = run_search(&grid, start, end);
            let score = distance[end];
            if score == u32::MAX {
                println!("Last byte position: {},{}", next_byte_pos[0], next_byte_pos[1]);
                return;
            }
            rebuild_path(start, end, parents, &mut path_members);
        }
    }
}

fn rebuild_path(
    start: usize,
    end: usize,
    parents: Vec<HashSet<usize>>,
    path_members: &mut Vec<bool>,
) {
    path_members.fill(false);
    path_members[end] = true;
    let mut look_at = end;
    while look_at != start {
        let parent = *parents[look_at].iter().take(1).next().unwrap();
        path_members[parent] = true;
        look_at = parent;
    }
}

fn run_search(grid: &Vec<char>, start: usize, end: usize) -> (Vec<u32>, Vec<HashSet<usize>>) {
    // remember distances from start
    let mut distance = vec![u32::MAX; grid.len()];
    // which nodes are parent to me
    let mut parents = vec![HashSet::new(); grid.len()];

    let heuristic = |pos: usize| {
        // commented out as it make the algorithm slower
        /*
        let end_row = end / GRID_SIZE;
        let end_col = end % GRID_SIZE;
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;
        let (end_row, end_col, row, col) = (end_row as f32, end_col as f32, row as f32, col as f32);
        let val = ((end_row - row).powi(2) + (end_col - col).powi(2)).sqrt();
        val as i64
         */
        0
    };

    // priority queue how to
    let mut queue = BinaryHeap::new();
    queue.push((0 - heuristic(start), 0, start, None));

    while !queue.is_empty() {
        let (_, cost, pos, parent_pos) = queue.pop().unwrap();

        // early exit if resulting score is higher than the achieved one
        if cost > distance[end] {
            break;
        }

        // skip if node is already visited
        if distance[pos] <= cost {
            continue;
        }

        // store current distance and link to parent
        distance[pos] = cost;
        if parent_pos.is_some() {
            parents[pos].insert(parent_pos.unwrap());
        }

        // expand
        let steps = vec![pos - GRID_SIZE, pos + GRID_SIZE, pos - 1, pos + 1];
        for next_pos in steps {
            if grid[next_pos] == '#' {
                continue;
            }
            let new_cost = cost + 1;
            queue.push((
                new_cost as i64 * (-1) - heuristic(next_pos),
                new_cost,
                next_pos,
                Some(pos),
            ));
        }
    }
    (distance, parents)
}

fn print_grid(grid: &Vec<char>) {
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            print!("{}", grid[i * GRID_SIZE + j]);
        }
        println!();
    }
}
