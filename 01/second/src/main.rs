fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    // create two vectors to store the two lists
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    lines.for_each(|line| {
        let mut numbers = line.split("   ");
        let left = numbers.next().unwrap().parse::<i32>().unwrap();
        let right = numbers.next().unwrap().parse::<i32>().unwrap();
        left_list.push(left);
        right_list.push(right);
    });

    // sort each line
    left_list.sort();
    right_list.sort();

    // sum each diff
    let mut left_pos = 0;
    let mut right_pos = 0;
    let mut right_count = 0;
    let mut sum = 0;

    while left_pos < left_list.len() {
        if left_list[left_pos] == right_list[right_pos] {
            right_count += 1;
            right_pos += 1;
            continue;
        }
        if left_list[left_pos] > right_list[right_pos] {
            right_pos += 1;
            continue;
        }

        let current_number = left_list[left_pos];
        while left_pos < left_list.len() && left_list[left_pos] == current_number {
            sum += left_list[left_pos] * right_count;
            left_pos += 1;
        }
        right_count = 0;
    }

    println!("{}", sum);
}
