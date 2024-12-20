use std::collections::{BinaryHeap, HashSet};

const MIN_SAVING: i32 = 100;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let height = input.lines().count();
    let grid: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let width = grid.len() / height;
    let end = grid.iter().position(|&c| c == 'E').unwrap();

    let distances = run_search(&grid, end, width).0;

    let part1 = find_cheats(&grid, width, &distances, 2);
    let part2 = find_cheats(&grid, width, &distances, 20);

    println!("{}", part1);
    println!("{}", part2);
}

fn find_cheats(grid: &Vec<char>, width: usize, distances: &Vec<i32>, max_cheat_cost: i32) -> usize {
    let mut possible_cheats = 0;

    for cheat_from in 0..grid.len() {
        if grid[cheat_from] == '#' {
            continue;
        }
        let cheat_from_row = (cheat_from / width) as i32;
        let cheat_from_col = (cheat_from % width) as i32;
        for cheat_to in 0..grid.len() {
            if grid[cheat_to] == '#' {
                continue;
            }
            let cheat_to_row = (cheat_to / width) as i32;
            let cheat_to_col = (cheat_to % width) as i32;

            let cheat_cost = i32::abs(cheat_from_row - cheat_to_row)
                + i32::abs(cheat_from_col - cheat_to_col);
            if cheat_cost > max_cheat_cost {
                continue;
            }

            let saving = distances[cheat_to] - distances[cheat_from] - cheat_cost;
            if saving < MIN_SAVING {
                continue;
            }

            possible_cheats += 1;
        }
    }

    possible_cheats
}

fn run_search(
    grid: &Vec<char>,
    start: usize,
    grid_width: usize,
) -> (Vec<i32>, Vec<HashSet<usize>>) {
    // remember distances from start
    let mut distance = vec![i32::MAX; grid.len()];
    // which nodes are parent to me
    let mut parents = vec![HashSet::new(); grid.len()];

    // priority queue how to
    let mut queue = BinaryHeap::new();
    queue.push((0, 0, start, None));

    while !queue.is_empty() {
        let (_, cost, pos, parent_pos) = queue.pop().unwrap();

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
        let steps = vec![pos - grid_width, pos + grid_width, pos - 1, pos + 1];
        for next_pos in steps {
            if grid[next_pos] == '#' {
                continue;
            }
            let new_cost = cost + 1;
            queue.push((new_cost as i64 * (-1), new_cost, next_pos, Some(pos)));
        }
    }
    (distance, parents)
}
