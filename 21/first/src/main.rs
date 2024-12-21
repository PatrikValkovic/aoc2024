use std::collections::HashSet;

const START_SYMBOL: char = 'A';

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        println!("{}", line);
        let number = line[..line.len() - 1].parse::<i32>().unwrap();
        let mut longest = i32::MAX;

        let first = get_moves(
            get_numpad_position(START_SYMBOL),
            &line.chars().collect(),
            get_numpad_position,
            fill_numpad_steps,
        );

        for l1 in first {
            let second = get_moves(
                get_keypad_position(START_SYMBOL),
                &l1,
                get_keypad_position,
                fill_keypad_steps,
            );

            for l2 in second {
                let third = get_moves(
                    get_keypad_position(START_SYMBOL),
                    &l2,
                    get_keypad_position,
                    fill_keypad_steps,
                );

                for l3 in third {
                    if l3.len() < longest as usize {
                        longest = l3.len() as i32;
                    }
                }
            }
        }


        println!("Complexity: {}", longest);
        sum += longest * number;
    }

    println!("{}", sum);
}

fn get_moves(
    start_position: (i32, i32),
    to_type: &Vec<char>,
    position_estimate: fn(char) -> (i32, i32),
    fill_steps: fn((i32, i32), (i32, i32)) -> Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut current_location = start_position;
    let mut all_variants: Vec<Vec<char>> = Vec::new();

    for symbol in to_type.iter() {
        let symbol = *symbol;
        let symbol_pos = position_estimate(symbol);

        let mut variants = fill_steps(current_location, symbol_pos);
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

fn fill_numpad_steps(from: (i32, i32), to: (i32, i32)) -> Vec<Vec<char>> {
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
            for _ in 0..horizontal_steps-1 {
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

fn fill_keypad_steps(from: (i32, i32), to: (i32, i32)) -> Vec<Vec<char>> {
    let mut variants = Vec::new();
    let vertical_steps = from.0 - to.0;
    let vertical_direction = if vertical_steps > 0 { '^' } else { 'v' };
    let vertical_steps = vertical_steps.abs();

    let horizontal_steps = from.1 - to.1;
    let horizontal_direction = if horizontal_steps > 0 { '<' } else { '>' };
    let horizontal_steps = horizontal_steps.abs();

    let stable_block1 = HashSet::from([
        (0, 1), (0, 2),
        (1, 1), (1, 2)
    ]);
    let stable_block2 = HashSet::from([
        (1, 0), (1, 1), (1, 2)
    ]);
    let stable_block3 = HashSet::from([
        (0, 1), (0, 2),
    ]);

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
