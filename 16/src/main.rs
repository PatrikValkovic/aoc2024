use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}
impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            3 => Direction::Down,
            _ => panic!("Invalid value"),
        }
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let lines = input.lines();
    let height = lines.clone().count();
    let grid: Vec<_> = lines.flat_map(|line| line.chars()).collect();
    let width = grid.len() / height;

    // find start and end
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(index, &c)| if c == 'S' { Some(index) } else { None })
        .unwrap();
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(index, &c)| if c == 'E' { Some(index) } else { None })
        .unwrap();
    // remember distances from start
    let mut distance = vec![u32::MAX; 4 * grid.len()];
    // which nodes are parent to me
    let mut parents = vec![HashSet::new(); 4 * grid.len()];

    let heuristic = |pos: usize| {
        // commended out, as it make it actually slower
        /*
        let end_row = end / width;
        let end_col = end % width;
        let row = pos / width;
        let col = pos % width;
        let (end_row, end_col, row, col) = (
            end_row as f32,
            end_col as f32,
            row as f32,
            col as f32,
        );
        let val = ((end_row - row).powi(2) + (end_col - col).powi(2)).sqrt();
        val as i64
         */
        return 0
    };

    // priority queue how to
    let mut queue = BinaryHeap::new();
    queue.push((0 - heuristic(start), 0, start, Direction::Right, None, Direction::Right));

    while !queue.is_empty() {
        let (_, cost, pos, direction, parent_pos, parent_direction) = queue.pop().unwrap();

        // early exit if resulting score is higher than the achieved one
        let end_score = *distance[end * 4..(end + 1) * 4].iter().min().unwrap();
        if cost > end_score {
            break;
        }

        // skip if node is already visited
        if distance[pos * 4 + direction as usize] < cost {
            continue;
        }

        // store current distance and link to parent
        distance[pos * 4 + direction as usize] = cost;
        if parent_pos.is_some() {
            parents[pos * 4 + direction as usize].insert((parent_pos.unwrap(), parent_direction));
        }

        // expand
        let steps = vec![
            (pos - width, Direction::Up),
            (pos + width, Direction::Down),
            (pos - 1, Direction::Left),
            (pos + 1, Direction::Right),
        ];
        for (next_pos, next_dir) in steps {
            if grid[next_pos] == '#' {
                continue;
            }
            let new_cost = cost + get_rotation_cost(direction, next_dir) + 1;
            queue.push((
                new_cost as i64 * (-1) - heuristic(next_pos),
                new_cost,
                next_pos,
                next_dir,
                Some(pos),
                direction,
            ));
        }
    }

    // get score
    let final_position_offset = distance[end * 4..(end + 1) * 4]
        .iter()
        .enumerate()
        .min_by_key(|(_, &x)| x)
        .map(|(i, _)| i)
        .unwrap();
    let final_position_index = end * 4 + final_position_offset;
    let score = distance[final_position_index];
    println!("Distance: {}", score);

    // backtrack
    let mut tiles = vec![0; grid.len()];
    let mut queue = vec![];
    queue.push((end, Direction::from(final_position_offset)));
    while !queue.is_empty() {
        let (pos, direction) = queue.pop().unwrap();
        tiles[pos] = 1;
        let my_parents = &parents[pos * 4 + direction as usize];
        for parent in my_parents.iter() {
            queue.push(*parent)
        }
    }
    let total_tiles: usize = tiles.iter().sum();
    println!("Tiles: {total_tiles}")
}

fn get_rotation_cost(prev_dir: Direction, current_dir: Direction) -> u32 {
    if prev_dir == current_dir {
        return 0;
    }
    if prev_dir == Direction::Up || prev_dir == Direction::Down {
        if current_dir == Direction::Up || current_dir == Direction::Down {
            return 2000;
        }
        return 1000;
    }
    if prev_dir == Direction::Left || prev_dir == Direction::Right {
        if current_dir == Direction::Left || current_dir == Direction::Right {
            return 2000;
        }
        return 1000;
    }
    return 0;
}
