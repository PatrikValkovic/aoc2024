use std::collections::{HashMap, HashSet};

const START_SYMBOL: char = 'A';
const MAX_DEPTH: usize = 25;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut cache = HashMap::new();

    let mut sum = 0;
    for line in lines {
        let number = line[..line.len() - 1].parse::<i32>().unwrap();
        let mut shortest_variant = usize::MAX;

        let first = get_numpad_moves(get_numpad_position(START_SYMBOL), &line.chars().collect());
        for numpad_variant in first.iter() {
            let mut variant_length = 0;
            for subsequence in numpad_variant
                .iter()
                .collect::<String>()
                .split(START_SYMBOL)
                .filter(|x| x.len() > 0)
            {
                let subsequence_with_action =
                    turn_into_move_with_action(&subsequence.chars().collect());
                let shortest_variant = get_shortest_keypad_sequence(
                    subsequence_with_action.as_str(),
                    0,
                    MAX_DEPTH,
                    &mut cache,
                );
                variant_length += shortest_variant;
            }
            if variant_length < shortest_variant {
                shortest_variant = variant_length;
            }
        }

        sum += shortest_variant * number as usize;
    }

    println!("{}", sum);
}

fn get_shortest_keypad_sequence(
    sequence: &str,
    current_depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if current_depth >= max_depth {
        return sequence.len();
    }

    if cache.contains_key(&(current_depth, String::from(sequence))) {
        return cache[&(current_depth, String::from(sequence))];
    }

    let mut current_position = get_keypad_position(START_SYMBOL);
    let mut current_length = 0;

    for next_symbol in sequence.chars() {
        let target_position = get_keypad_position(next_symbol);
        let possible_moves = generate_keypad_steps(current_position, target_position);
        let mut shortest_for_current_position = usize::MAX;
        for possible_move in possible_moves.iter() {
            let with_action = turn_into_move_with_action(possible_move);
            let shortest_for_possible_move = get_shortest_keypad_sequence(
                with_action.as_str(),
                current_depth + 1,
                max_depth,
                cache,
            );
            if shortest_for_possible_move < shortest_for_current_position {
                shortest_for_current_position = shortest_for_possible_move;
            }
        }
        current_length += shortest_for_current_position;
        current_position = target_position;
    }

    cache.insert((current_depth, String::from(sequence)), current_length);
    return current_length;
}

fn get_numpad_moves(start_position: (i32, i32), to_type: &Vec<char>) -> Vec<Vec<char>> {
    let mut current_location = start_position;
    let mut all_variants: Vec<Vec<char>> = Vec::new();

    for symbol in to_type.iter() {
        let symbol = *symbol;
        let symbol_pos = get_numpad_position(symbol);

        let mut variants = generate_numpad_steps(current_location, symbol_pos);
        for variant in variants.iter_mut() {
            variant.push('A')
        }

        let mut new_variants = Vec::new();
        for variant in variants.iter() {
            for previous_variant in all_variants.iter() {
                let mut new_variant = previous_variant.clone();
                new_variant.append(&mut variant.clone());
                new_variants.push(new_variant);
            }
        }
        if new_variants.len() == 0 {
            new_variants = variants;
        }

        all_variants = new_variants;
        current_location = symbol_pos;
    }

    let mut deduplicated = HashSet::new();
    for variant in all_variants.iter() {
        deduplicated.insert(variant.iter().collect::<String>());
    }

    return deduplicated.iter().map(|x| x.chars().collect()).collect();
}

fn generate_numpad_steps(from: (i32, i32), to: (i32, i32)) -> Vec<Vec<char>> {
    let mut variants = Vec::new();
    let vertical_steps = from.0 - to.0;
    let vertical_direction = if vertical_steps > 0 { '^' } else { 'v' };
    let vertical_steps = vertical_steps.abs();

    let horizontal_steps = from.1 - to.1;
    let horizontal_direction = if horizontal_steps > 0 { '<' } else { '>' };
    let horizontal_steps = horizontal_steps.abs();

    let stable_blocks1 = HashSet::from([
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ]);
    let stable_blocks2 = HashSet::from([
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 1),
        (3, 2),
        (4, 1),
        (4, 2),
    ]);
    let stable_blocks3 = HashSet::from([(0, 0), (0, 1), (0, 2)]);
    let stable_block4 = HashSet::from([(3, 1), (3, 2)]);
    if stable_blocks1.contains(&from) && stable_blocks1.contains(&to)
        || stable_blocks2.contains(&from) && stable_blocks2.contains(&to)
        || stable_blocks3.contains(&from) && stable_blocks3.contains(&to)
        || stable_block4.contains(&from) && stable_block4.contains(&to)
    {
        let mut vertical_variant = Vec::new();
        for _ in 0..vertical_steps {
            vertical_variant.push(vertical_direction);
        }
        for _ in 0..horizontal_steps {
            vertical_variant.push(horizontal_direction);
        }
        variants.push(vertical_variant);

        let mut horizontal_variant = Vec::new();
        for _ in 0..horizontal_steps {
            horizontal_variant.push(horizontal_direction);
        }
        for _ in 0..vertical_steps {
            horizontal_variant.push(vertical_direction);
        }
        variants.push(horizontal_variant);

        let mut deduplicated = HashSet::new();
        for variant in variants.iter() {
            deduplicated.insert(variant.iter().collect::<String>());
        }

        return deduplicated.iter().map(|x| x.chars().collect()).collect();
    }

    // now we know that source is in first column and target in last row, or the other way around
    if from.0 == 3 {
        // source is in the last row
        if from.1 == 1 {
            // going from 0, must prefer horizontal step
            let mut horizontal_variant = Vec::new();
            for _ in 0..horizontal_steps {
                horizontal_variant.push(horizontal_direction);
            }
            for _ in 0..vertical_steps {
                horizontal_variant.push(vertical_direction);
            }
            variants.push(horizontal_variant);
        } else {
            // going from A
            let mut vertical_variant = Vec::new();
            for _ in 0..vertical_steps {
                vertical_variant.push(vertical_direction);
            }
            for _ in 0..horizontal_steps {
                vertical_variant.push(horizontal_direction);
            }
            variants.push(vertical_variant);

            let mut first_step_horizontal = Vec::new();
            first_step_horizontal.push(horizontal_direction);
            for _ in 0..vertical_steps {
                first_step_horizontal.push(vertical_direction);
            }
            for _ in 0..horizontal_steps - 1 {
                first_step_horizontal.push(horizontal_direction);
            }
        }
    } else {
        // source is in first column
        let mut horizontal_variant = Vec::new();
        for _ in 0..horizontal_steps {
            horizontal_variant.push(horizontal_direction);
        }
        for _ in 0..vertical_steps {
            horizontal_variant.push(vertical_direction);
        }
        variants.push(horizontal_variant);

        let possible_vertical_steps = 2 - from.0;
        if possible_vertical_steps > 0 {
            let mut first_step_vertical = Vec::new();
            for _ in 0..possible_vertical_steps {
                first_step_vertical.push(vertical_direction);
            }
            for _ in 0..horizontal_steps {
                first_step_vertical.push(horizontal_direction);
            }
            for _ in 0..vertical_steps - possible_vertical_steps {
                first_step_vertical.push(vertical_direction);
            }
            variants.push(first_step_vertical);
        }
    }

    let mut deduplicated = HashSet::new();
    for variant in variants.iter() {
        deduplicated.insert(variant.iter().collect::<String>());
    }

    return deduplicated.iter().map(|x| x.chars().collect()).collect();
}

fn generate_keypad_steps(from: (i32, i32), to: (i32, i32)) -> Vec<Vec<char>> {
    let mut variants = Vec::new();
    let vertical_steps = from.0 - to.0;
    let vertical_direction = if vertical_steps > 0 { '^' } else { 'v' };
    let vertical_steps = vertical_steps.abs();

    let horizontal_steps = from.1 - to.1;
    let horizontal_direction = if horizontal_steps > 0 { '<' } else { '>' };
    let horizontal_steps = horizontal_steps.abs();

    let stable_block1 = HashSet::from([(0, 1), (0, 2), (1, 1), (1, 2)]);
    let stable_block2 = HashSet::from([(1, 0), (1, 1), (1, 2)]);
    let stable_block3 = HashSet::from([(0, 1), (0, 2)]);

    if stable_block1.contains(&from) && stable_block1.contains(&to)
        || stable_block2.contains(&from) && stable_block2.contains(&to)
        || stable_block3.contains(&from) && stable_block3.contains(&to)
    {
        let mut vertical_variant = Vec::new();
        for _ in 0..vertical_steps {
            vertical_variant.push(vertical_direction);
        }
        for _ in 0..horizontal_steps {
            vertical_variant.push(horizontal_direction);
        }
        variants.push(vertical_variant);

        let mut horizontal_variant = Vec::new();
        for _ in 0..horizontal_steps {
            horizontal_variant.push(horizontal_direction);
        }
        for _ in 0..vertical_steps {
            horizontal_variant.push(vertical_direction);
        }
        variants.push(horizontal_variant);

        let mut deduplicated = HashSet::new();
        for variant in variants.iter() {
            deduplicated.insert(variant.iter().collect::<String>());
        }

        return deduplicated.iter().map(|x| x.chars().collect()).collect();
    }

    if from.0 == 1 {
        // source is <
        let mut horizontal_variant = Vec::new();
        for _ in 0..horizontal_steps {
            horizontal_variant.push(horizontal_direction);
        }
        for _ in 0..vertical_steps {
            horizontal_variant.push(vertical_direction);
        }
        variants.push(horizontal_variant);
    } else {
        // target is <
        let mut vertical_variant = Vec::new();
        for _ in 0..vertical_steps {
            vertical_variant.push(vertical_direction);
        }
        for _ in 0..horizontal_steps {
            vertical_variant.push(horizontal_direction);
        }
        variants.push(vertical_variant);

        let possible_left_steps = from.1 - 1;
        if possible_left_steps > 0 {
            let mut first_step_left = Vec::new();
            for _ in 0..possible_left_steps {
                first_step_left.push(horizontal_direction);
            }
            for _ in 0..vertical_steps {
                first_step_left.push(vertical_direction);
            }
            for _ in 0..horizontal_steps - possible_left_steps {
                first_step_left.push(horizontal_direction);
            }
            variants.push(first_step_left);
        }
    }

    let mut deduplicated = HashSet::new();
    for variant in variants.iter() {
        deduplicated.insert(variant.iter().collect::<String>());
    }

    return deduplicated.iter().map(|x| x.chars().collect()).collect();
}

fn get_keypad_position(symbol: char) -> (i32, i32) {
    match symbol {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!("Invalid symbol"),
    }
}

fn get_numpad_position(symbol: char) -> (i32, i32) {
    match symbol {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!("Invalid symbol"),
    }
}

fn turn_into_move_with_action(possible_move: &Vec<char>) -> String {
    let mut with_action = String::with_capacity(possible_move.len() + 1);
    possible_move.iter().for_each(|x| with_action.push(*x));
    with_action.push(START_SYMBOL);
    with_action
}
