fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut lines = input.lines();

    let mut tokens_spend = 0;
    while true {
        // parse info for current claw machine
        let a_button_string = lines.next().unwrap();
        let b_button_string = lines.next().unwrap();
        let prize_string = lines.next().unwrap();
        let mut a_movements = a_button_string.split(":").nth(1).unwrap().split(",").map(|x| x.trim());
        let a_x_move: f32 = a_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let a_y_move: f32 = a_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let mut b_movements = b_button_string.split(":").nth(1).unwrap().split(",").map(|x| x.trim());
        let b_x_move: f32 = b_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let b_y_move: f32 = b_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let mut price_locations = prize_string.split(":").nth(1).unwrap().split(",").map(|x| x.trim());
        let price_x: f32 = price_locations.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
        let price_y: f32 = price_locations.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();

        // solve equations
        let count_b =  (a_x_move*price_y - a_y_move * price_x)
            /
            (b_y_move * a_x_move - b_x_move * a_y_move);
        let count_a = (price_x - count_b * b_x_move) / a_x_move;
        let rounded_b = count_b.round();
        let rounded_a = count_a.round();

        // check results
        if count_b >= 0.0 && count_b <= 100.0 &&
            count_b >= 0.0 && count_b <= 100.0 &&
            f32::abs(count_a - rounded_a) <= f32::EPSILON &&
            f32::abs(count_b - rounded_b) <= f32::EPSILON {
            let count_b = count_b as u32;
            let count_a = count_a as u32;
            tokens_spend += count_a * 3 + count_b;
        }

        if let None = lines.next() {
            break;
        }
    }

    println!("{}", tokens_spend);
}
