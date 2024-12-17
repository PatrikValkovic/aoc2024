use regex::Regex;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let empty_line_index = lines.clone().position(|line| line.is_empty()).unwrap();
    // read registers
    let mut registers = vec![0; 3];
    let register_regex = Regex::new(r"Register (?<RegisterName>[A-C]): (?<Value>[0-9]*)").unwrap();
    lines.clone().take(empty_line_index).for_each(|line| {
        let captures = register_regex.captures(line).unwrap();
        let register_name = captures.name("RegisterName").unwrap().as_str();
        let value = captures
            .name("Value")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        match register_name {
            "A" => registers[0] = value,
            "B" => registers[1] = value,
            "C" => registers[2] = value,
            _ => panic!("Invalid register name"),
        }
    });
    // read instructions
    let instructions: Vec<u32> = lines
        .skip(empty_line_index + 1)
        .flat_map(|line| {
            let instruction_numbers = line["Program:".len()..].trim();
            return instruction_numbers
                .split(",")
                .map(|number| number.parse().unwrap());
        })
        .collect();

    // prepare variables
    let mut instruction_pointer = 0;
    let mut output = Vec::new();
    while instruction_pointer < instructions.len() {
        let instruction = instructions[instruction_pointer];
        let operand = instructions[instruction_pointer + 1];

        match instruction {
            0 => {
                let numerator = registers[0] as f32;
                let denominator = 2f32.powf(evaluate_combo_operand(operand, &registers) as f32);
                registers[0] = (numerator / denominator) as i32;
            }
            1 => {
                registers[1] ^= operand as i32;
            }
            2 => {
                registers[1] = evaluate_combo_operand(operand, &registers) % 8;
            }
            3 => {
                if registers[0] != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => {
                registers[1] ^= registers[2];
            }
            5 => {
                output.push(evaluate_combo_operand(operand, &registers) % 8);
            }
            6 => {
                let numerator = registers[0] as f32;
                let denominator = 2f32.powf(evaluate_combo_operand(operand, &registers) as f32);
                registers[1] = (numerator / denominator) as i32;
            }
            7 => {
                let numerator = registers[0] as f32;
                let denominator = 2f32.powf(evaluate_combo_operand(operand, &registers) as f32);
                registers[2] = (numerator / denominator) as i32;
            }
            _ => panic!("Invalid instruction"),
        }

        instruction_pointer += 2;
    }

    println!(
        "{}",
        output
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn evaluate_combo_operand(operant: u32, registers: &Vec<i32>) -> i32 {
    match operant {
        0..=3 => operant as i32,
        4..=6 => registers[(operant - 4) as usize],
        _ => panic!("Invalid combo operand"),
    }
}
