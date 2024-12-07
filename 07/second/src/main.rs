fn operation_plus(a: i64, b: i64) -> i64 {
    a + b
}

fn operation_multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn operation_concat(a: i64, b: i64) -> i64 {
    let b_positions = if b > 0 { b.ilog10() + 1 } else { 1 };
    a * 10i64.pow(b_positions) + b
}

fn find_expression(numbers: &[i64], target: i64, current: i64) -> bool {
    if numbers.is_empty() {
        return if current == target { true } else { false };
    }

    if current > target {
        return false;
    }

    let num = numbers[0];
    let remaining_numbers = &numbers[1..];

    // recursively call with given operations
    if find_expression(remaining_numbers, target, operation_multiply(current, num)) {
        return true;
    }
    if find_expression(remaining_numbers, target, operation_plus(current, num)) {
        return true;
    }
    if find_expression(remaining_numbers, target, operation_concat(current, num)) {
        return true;
    }

    false
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let mut true_equations = 0;
    for line in lines {
        let mut split_by_colon = line.split(": ");
        let result = split_by_colon.next().unwrap().parse::<i64>().unwrap();
        let numbers = split_by_colon
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        // use backtracking
        if find_expression(&numbers[1..], result, numbers[0]) {
            true_equations += result;
        }
    }

    println!("{}", true_equations);
}
