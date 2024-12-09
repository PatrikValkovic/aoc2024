fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let numbers: Vec<usize> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let total_cells = numbers.iter().sum();
    let mut blocks = Vec::with_capacity(total_cells);

    // input into blocks
    let mut is_file = true;
    let mut file_index = 0;
    for block_length in numbers {
        if is_file {
            for _ in 0..block_length {
                blocks.push(Some(file_index));
            }
            file_index += 1;
        } else {
            for _ in 0..block_length {
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }

    // compress
    let mut left_pos = 0;
    let mut right_pos = blocks.len() - 1;
    while left_pos < right_pos {
        if !blocks[left_pos].is_none() {
            left_pos += 1;
            continue;
        }
        if blocks[right_pos].is_none() {
            right_pos -= 1;
            continue;
        }
        blocks[left_pos] = blocks[right_pos];
        blocks[right_pos] = None;
        left_pos += 1;
        right_pos -= 1;
    }

    // calculate checksum
    let checksum: usize = blocks.iter().enumerate().map(|(i, block)| {
        match block {
            Some(file_id) => i * file_id,
            None => 0,
        }
    }).sum();

    println!("{}", checksum);
}
