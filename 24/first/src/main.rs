use regex::Regex;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let empty_line_index = lines.iter().position(|&x| x == "").unwrap();

    let mut wire_states = std::collections::HashMap::new();
    let mut waiting_for_wire = std::collections::HashMap::new();
    let mut wires_to_process = Vec::new();

    let gate_regex = Regex::new("(?<left>[a-z0-9]+) (?<operator>AND|XOR|OR) (?<right>[a-z0-9]+) -> (?<result>[a-z0-9]+)").unwrap();
    for i in empty_line_index+1..lines.len() {
        let line = lines[i];
        let captures = gate_regex.captures(line).unwrap();
        let left = captures.name("left").unwrap().as_str();
        let right = captures.name("right").unwrap().as_str();
        let operator = captures.name("operator").unwrap().as_str();
        let result = captures.name("result").unwrap().as_str();
        let record = (left, right, operator, result);
        waiting_for_wire.entry(left).or_insert(vec![]).push(record);
        waiting_for_wire.entry(right).or_insert(vec![]).push(record);
    }

    for i in 0..empty_line_index {
        let line = lines[i];
        let mut parts = line.split(": ");
        let wire = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim().parse::<u8>().unwrap();
        wire_states.insert(wire, value > 0);
        wires_to_process.push(wire);
    }

    while !wires_to_process.is_empty() {
        let processed_wire = wires_to_process.pop().unwrap();
        let dependant_lines = waiting_for_wire.remove(&processed_wire).or(Some(vec![])).unwrap();
        for (left, right, operator, result) in dependant_lines {
            let left_value = wire_states.get(left);
            let right_value = wire_states.get(right);
            if left_value.is_none() || right_value.is_none() {
                continue;
            }
            let left_value = *left_value.unwrap();
            let right_value = *right_value.unwrap();
            let result_value = match operator {
                "AND" => left_value && right_value,
                "XOR" => left_value != right_value,
                "OR" => left_value || right_value,
                _ => panic!("Unknown operator")
            };
            wire_states.insert(result, result_value);
            wires_to_process.push(result);
        }
    }

    let mut result: u64 = 0;
    for (wire, value) in wire_states {
        if wire.starts_with("z") {
            let index = wire[1..].parse::<u64>().unwrap();
            result |= (value as u64) << index;
        }
    }
    
    println!("{}", result);
}
