const COLUMNS: usize = 5;
const ROWS: usize = 7;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut parts = input.split("\n\n");
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while let Some(part) = parts.next() {
        let parse_result = parse_lock_or_key(part);
        match parse_result {
            ParseResult::Lock(lock) => locks.push(lock),
            ParseResult::Key(key) => keys.push(key),
        };
    }

    let mut possible_combinations: usize = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let is_possible = can_be_combined(lock, key);
            if is_possible {
                possible_combinations += 1;
            }
        }
    }

    println!("{}", possible_combinations);
}

fn can_be_combined(lock: &Vec<i32>, key: &Vec<i32>) -> bool {
    let mut is_possible = true;
    for i in 0..5 {
        is_possible = is_possible && lock[i] + key[i] <= 5;
    }
    is_possible
}

enum ParseResult {
    Lock(Vec<i32>),
    Key(Vec<i32>),
}

fn parse_lock_or_key(part: &str) -> ParseResult {
    let mut buffer = vec![vec!['.'; COLUMNS]; ROWS];
    let lines = part.lines();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            buffer[i][j] = c;
        }
    }

    let is_lock = buffer[0][0] == '#';

    let mut pattern = vec![];
    for i in 0..5 {
        let mut count: i32 = 0;
        for j in 1..7 {
            if buffer[j][i] == buffer[j - 1][i] {
                count += 1;
            } else {
                break;
            }
        }
        pattern.push(count);
    }

    if !is_lock {
        pattern = pattern.iter().map(|v| ROWS as i32 - 2 - *v).collect();
    }

    if is_lock {
        ParseResult::Lock(pattern)
    } else {
        ParseResult::Key(pattern)
    }
}
