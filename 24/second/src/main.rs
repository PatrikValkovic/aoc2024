use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Eq, PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    fn evaluate(&self, left: bool, right: bool) -> bool {
        match self {
            Operator::AND => left && right,
            Operator::OR => left || right,
            Operator::XOR => left != right,
        }
    }

    fn from_str(operator: &str) -> Operator {
        match operator {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            _ => panic!("Unknown operator"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Operator::AND => "AND",
            Operator::OR => "OR",
            Operator::XOR => "XOR",
        }
    }
}

struct WireValueNode {
    wire: String,
    value: bool,
}
struct WireOperationNode {
    result: WireValueNode,
    left: Rc<WireNode>,
    right: Rc<WireNode>,
    operator: Operator,
}
enum WireNode {
    Value(WireValueNode),
    Operation(WireOperationNode),
}

impl WireValueNode {
    fn new(wire: String, value: bool) -> WireNode {
        WireNode::Value(WireValueNode { wire, value })
    }
}
impl WireOperationNode {
    fn new(
        left: &Rc<WireNode>,
        right: &Rc<WireNode>,
        operator: Operator,
        wire: &str,
        value: bool,
    ) -> WireNode {
        let result = WireValueNode {
            wire: wire.to_string(),
            value,
        };
        WireNode::Operation(WireOperationNode {
            result,
            left: Rc::clone(left),
            right: Rc::clone(right),
            operator,
        })
    }

    fn render_topology(&self, max_depth: usize) -> String {
        let mut content = String::with_capacity(10);
        let mut blocked = Vec::with_capacity(8);
        self._render_topology(&mut content, &mut blocked, 0, max_depth);
        content
    }

    fn _render_topology(
        &self,
        content: &mut String,
        blocked: &mut Vec<bool>,
        depth: usize,
        max_depth: usize,
    ) {
        content.push_str(self.result.wire.as_str());
        if depth >= max_depth {
            return;
        }

        content.push_str("\n");
        blocked.push(true);

        Self::_render_tree_prefix(content, blocked, depth);
        content.push_str("├─");
        if let WireNode::Value(node) = self.left.as_ref() {
            content.push_str(node.wire.as_str());
        } else {
            if let WireNode::Operation(node) = self.left.as_ref() {
                node._render_topology(content, blocked, depth + 1, max_depth);
            }
        }

        content.push_str("\n");
        Self::_render_tree_prefix(content, blocked, depth);
        content.push_str("├");
        content.push_str(self.operator.to_str());
        content.push_str("\n");
        Self::_render_tree_prefix(content, blocked, depth);
        content.push_str("└─");
        blocked[depth] = false;

        if let WireNode::Value(node) = self.right.as_ref() {
            content.push_str(node.wire.as_str());
        } else {
            if let WireNode::Operation(node) = self.right.as_ref() {
                node._render_topology(content, blocked, depth + 1, max_depth);
            }
        }

        blocked.pop();
    }

    fn _render_tree_prefix(content: &mut String, blocked: &mut Vec<bool>, depth: usize) {
        for i in 0..depth {
            if blocked[i] {
                content.push_str("│ ");
            } else {
                content.push_str("  ");
            }
        }
    }
}
impl WireNode {
    fn get_value(&self) -> bool {
        match self {
            WireNode::Value(node) => node.value,
            WireNode::Operation(node) => node.result.value,
        }
    }
    fn to_value_node<'a>(&'a self) -> &'a WireValueNode {
        match self {
            WireNode::Value(node) => node,
            WireNode::Operation(node) => &node.result,
        }
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let empty_line_index = lines.iter().position(|&x| x == "").unwrap();
    let node_lines = lines[empty_line_index + 1..]
        .iter()
        .map(|x| *x)
        .collect::<Vec<_>>();

    let mut wires = std::collections::HashMap::new();
    let mut waiting_for_wire = std::collections::HashMap::new();
    let mut wires_to_process = Vec::new();

    let gate_regex = Regex::new(
        "(?<left>[a-z0-9]+) (?<operator>AND|XOR|OR) (?<right>[a-z0-9]+) -> (?<result>[a-z0-9]+)",
    )
    .unwrap();
    for line in node_lines.iter() {
        let line = *line;
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
        let node = WireValueNode::new(wire.to_string(), value > 0);
        wires.insert(wire, Rc::new(node));
        wires_to_process.push(wire);
    }

    while !wires_to_process.is_empty() {
        let processed_wire = wires_to_process.pop().unwrap();
        let dependant_lines = waiting_for_wire
            .remove(&processed_wire)
            .or(Some(vec![]))
            .unwrap();
        for (left, right, operator, result_wire) in dependant_lines {
            let left_value = wires.get(left);
            let right_value = wires.get(right);
            if left_value.is_none() || right_value.is_none() {
                continue;
            }
            let left_node = left_value.unwrap();
            let right_node = right_value.unwrap();
            let left_value = left_node.as_ref().get_value();
            let right_value = right_node.as_ref().get_value();
            let operator = Operator::from_str(operator);
            let result_value = operator.evaluate(left_value, right_value);
            let node =
                WireOperationNode::new(left_node, right_node, operator, result_wire, result_value);
            wires.insert(result_wire, Rc::new(node));
            wires_to_process.push(result_wire);
        }
    }

    let x_number: u64 = get_wireset_result(&wires, "x");
    let y_number: u64 = get_wireset_result(&wires, "y");
    let z_number = get_wireset_result(&wires, "z");
    let expected_result = x_number + y_number;
    println!("Current result: {}", z_number);

    println!("Actual:   {:b}", z_number);
    println!("Expected: {:b}", expected_result);

    let mismatch_positions = get_mismatch_positions(z_number, expected_result);
    println!("Changes at: {:?}", mismatch_positions);

    let mut sorted_z_keys = wires
        .keys()
        .filter_map(|key| {
            if key.starts_with("z") {
                Some(*key)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    sorted_z_keys.sort();
    for key in sorted_z_keys {
        let node = wires.get(key).unwrap();
        if let WireNode::Operation(node) = node.as_ref() {
            println!("{}", node.render_topology(usize::MAX));
        }
    }

    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    let mut xor = Vec::new();
    let mut add = Vec::new();
    let mut over = Vec::new();
    let mut carry = Vec::new();

    for i in 0..64 {
        let x_node = wires.get(format!("x{:02}", i).as_str());
        let y_node = wires.get(format!("y{:02}", i).as_str());
        let z_node = wires.get(format!("z{:02}", i).as_str());
        if x_node.is_none() || y_node.is_none() || z_node.is_none() {
            continue;
        }
        let x_node = x_node.unwrap().to_value_node();
        let y_node = y_node.unwrap().to_value_node();
        let z_node = z_node.unwrap().to_value_node();
        x.push(x_node.wire.as_str());
        y.push(y_node.wire.as_str());
        z.push(z_node.wire.as_str());

        let add_node = find_node(&wires, x[i], y[i], Operator::AND);
        if add_node.is_none() {
            panic!(
                "ADD node for {} is missing, expected {} {} {}",
                i, x[i], "AND", y[i],
            );
        }
        let add_node = add_node.unwrap().1.to_value_node();
        add.push(add_node.wire.as_str());

        let xor_node = find_node(&wires, x[i], y[i], Operator::XOR);
        if xor_node.is_none() {
            panic!(
                "XOR node for {} is missing, expected {} {} {}",
                i, x[i], "XOR", y[i],
            );
        }
        let xor_node = xor_node.unwrap().1.to_value_node();
        xor.push(xor_node.wire.as_str());

        let over_node = if i > 0 {
            let over_node = find_node(&wires, xor[i], carry[i - 1], Operator::AND);
            if over_node.is_none() {
                panic!(
                    "OVER node for {} is missing, expected {} {} {}",
                    i,
                    xor[i],
                    "AND",
                    carry[i - 1],
                );
            }
            over_node.unwrap().1.to_value_node().wire.as_str()
        } else {
            xor[i]
        };
        over.push(over_node);

        let carry_node = if i > 0 {
            let carry_node = find_node(&wires, add[i], over[i], Operator::OR);
            if carry_node.is_none() {
                panic!(
                    "CARRY node for {} is missing, expected {} {} {}",
                    i,
                    add[i],
                    "OR",
                    over[i],
                );
            }
            carry_node.unwrap().1.to_value_node().wire.as_str()
        } else {
            add[i]
        };
        carry.push(carry_node);
    }
}

fn find_node<'a>(
    wires: &'a HashMap<&'a str, Rc<WireNode>>,
    left_side: &'a str,
    right_side: &'a str,
    operation: Operator,
) -> Option<(&'a &'a str, &'a Rc<WireNode>)> {
    wires.iter().find(|(_, node)| match node.as_ref() {
        WireNode::Operation(node) => {
            let is_and_operator = node.operator == operation;
            let has_x = node.left.as_ref().to_value_node().wire == left_side
                || node.right.as_ref().to_value_node().wire == left_side;
            let has_y = node.left.as_ref().to_value_node().wire == right_side
                || node.right.as_ref().to_value_node().wire == right_side;
            return is_and_operator && has_x && has_y;
        }
        _ => false,
    })
}

fn get_wireset_result(wires: &HashMap<&str, Rc<WireNode>>, prefix: &str) -> u64 {
    let mut result: u64 = 0;
    for (wire, value) in wires.iter() {
        if let WireNode::Operation(value) = value.as_ref() {
            if wire.starts_with(prefix) {
                let index = wire[1..].parse::<u64>().unwrap();
                result |= (value.result.value as u64) << index;
            }
        }
        if let WireNode::Value(value) = value.as_ref() {
            if wire.starts_with(prefix) {
                let index = wire[1..].parse::<u64>().unwrap();
                result |= (value.value as u64) << index;
            }
        }
    }
    result
}

fn get_mismatch_positions(a: u64, b: u64) -> Vec<usize> {
    let mut positions = Vec::new();
    for i in 0..64 {
        let mask = 1 << i;
        if (a & mask) != (b & mask) {
            positions.push(i);
        }
    }
    positions
}
