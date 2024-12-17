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
            .parse::<i64>()
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

    let val = perform_search(
        0,
        (instructions.len() - 1) as i64,
        &mut registers,
        &instructions,
    );

    println!("{}", val);
}

fn perform_search(val: i64, digit: i64, registers: &mut Vec<i64>, instructions: &Vec<u32>) -> i64 {
    if digit < 0 {
        return val;
    }

    let mut results = vec![i64::MAX];

    for i in 0..8 {
        let tmp = val | (i << (digit * 3));
        registers[0] = tmp;
        let output = evaluate(&instructions, &registers);

        if output.len() == instructions.len()
            && output[digit as usize] == instructions[digit as usize]
        {
            results.push(perform_search(
                val | (i << (digit * 3)),
                digit - 1,
                registers,
                instructions,
            ));
        }
    }

    return *results.iter().min().unwrap();
}

fn evaluate(instructions: &Vec<u32>, registers: &Vec<i64>) -> Vec<u32> {
    // prepare variables
    let mut registers = registers.clone();
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    // run the program
    while instruction_pointer < instructions.len() {
        let instruction = instructions[instruction_pointer];
        let operand = instructions[instruction_pointer + 1];

        match instruction {
            0 => {
                let numerator = registers[0];
                let denominator = 1 << evaluate_combo_operand(operand, &registers);
                registers[0] = numerator / denominator;
            }
            1 => {
                registers[1] ^= operand as i64;
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
                output.push((evaluate_combo_operand(operand, &registers) % 8) as u32);
            }
            6 => {
                let numerator = registers[0];
                let denominator = 1 << evaluate_combo_operand(operand, &registers);
                registers[1] = numerator / denominator;
            }
            7 => {
                let numerator = registers[0];
                let denominator = 1 << evaluate_combo_operand(operand, &registers);
                registers[2] = numerator / denominator;
            }
            _ => panic!("Invalid instruction"),
        }

        instruction_pointer += 2;
    }

    return output;
}

fn evaluate_combo_operand(operant: u32, registers: &Vec<i64>) -> i64 {
    match operant {
        0..=3 => operant as i64,
        4..=6 => registers[(operant - 4) as usize],
        _ => panic!("Invalid combo operand"),
    }
}
