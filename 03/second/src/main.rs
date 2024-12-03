fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();

    // define regex
    let re = regex::Regex::new(r"(?P<instruction>mul\((?P<first_number>\d+),\s*(?P<second_number>\d+)\)|do\(\)|don't\(\))").unwrap();

    // find all the occurrences
    let mut sum = 0;
    let mut instructions_enabled = true;
    for cap in re.captures_iter(&input) {
        match &cap["instruction"] {
            "do()" => instructions_enabled = true,
            "don't()" => instructions_enabled = false,
            _ => {
                if instructions_enabled {
                    let x: i32 = cap["first_number"].parse().unwrap();
                    let y: i32 = cap["second_number"].parse().unwrap();
                    sum += x * y;
                }
            }
        }
    }

    println!("{}", sum);
}
