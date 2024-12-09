#[derive(Clone)]
struct Block {
    pub file_id: Option<usize>,
    pub length: usize,
}

impl Block {
    fn new(file_id: Option<usize>, length: usize) -> Self {
        Block { file_id, length }
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let numbers: Vec<usize> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut blocks = Vec::with_capacity(numbers.len());

    // input into blocks
    let mut is_file = true;
    let mut file_index = 0;
    for block_length in numbers {
        if is_file {
            blocks.push(Block::new(Some(file_index), block_length));
            file_index += 1;
        } else {
            blocks.push(Block::new(None, block_length));
        }
        is_file = !is_file;
    }

    // compress
    render(&blocks);
    let mut right_pos = blocks.len() - 1;
    if blocks[right_pos].file_id.is_none() {
        right_pos -= 1;
    }
    while right_pos > 0 {
        let block_to_move = blocks[right_pos].clone();
        let free_block = blocks[..right_pos]
            .iter()
            .enumerate()
            .find_map(|(i, block)| {
                if block.file_id.is_none() && block.length >= block_to_move.length {
                    Some(i)
                } else {
                    None
                }
            });
        if free_block.is_none() {
            right_pos -= 1;
            continue;
        }
        let free_block = free_block.unwrap();
        if blocks[free_block].length == block_to_move.length {
            blocks.swap(free_block, right_pos);
        } else {
            let free_block_size = blocks[free_block].length;
            blocks[free_block] = Block::new(block_to_move.file_id, block_to_move.length);
            blocks[right_pos] = Block::new(None, block_to_move.length);
            blocks.insert(
                free_block + 1,
                Block::new(None, free_block_size - block_to_move.length),
            );
            right_pos += 1;
        }
        render(&blocks);
        if right_pos > 0 && blocks[right_pos - 1].file_id.is_none() {
            right_pos -= 1;
            blocks[right_pos] = Block::new(
                None,
                blocks[right_pos].length + blocks[right_pos + 1].length,
            );
            blocks.remove(right_pos + 1);
        }
        if right_pos + 1 < blocks.len() && blocks[right_pos + 1].file_id.is_none() {
            blocks[right_pos] = Block::new(
                None,
                blocks[right_pos].length + blocks[right_pos + 1].length,
            );
            blocks.remove(right_pos + 1);
        }
        render(&blocks);
        right_pos -= 1;
    }

    let mut checksum = 0;
    let mut index = 0;
    for block in blocks.iter() {
        match block.file_id {
            Some(file_id) => {
                for i in 0..block.length {
                    checksum += (index + i) * file_id;
                }
                index += block.length;
            }
            None => {
                index += block.length;
            }
        }
    }

    println!("{}", checksum);
}

const PRINT: bool = false;
fn render(blocks: &Vec<Block>) {
    if !PRINT {
        return;
    }
    for block in blocks {
        if block.file_id.is_some() {
            for _ in 0..block.length {
                print!("{}", block.file_id.unwrap());
            }
        } else {
            print!("|");
            for _ in 0..block.length {
                print!(".");
            }
            print!("|");
        }
    }
    println!()
}
