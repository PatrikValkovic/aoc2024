fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines.iter().flat_map(|x| x.chars()).collect::<Vec<char>>();
    let mut signals = vec![false; width * height];

    // create dictionary from antennas
    let mut antennas = std::collections::HashMap::new();
    for i in 0..width*height {
        let c = grid[i];
        if c == '.' {
            continue
        }
        if !antennas.contains_key(&c) {
            antennas.insert(c, Vec::new());
        }
        antennas.get_mut(&c).unwrap().push(i);
    }

    // calculate antinodes
    for (_, v) in antennas.iter() {
        for i in 1..v.len() {
            for j in 0..i {
                let i1 = v[i];
                let i2 = v[j];
                let y1 = (i1 / width) as f32;
                let x1 = (i1 % width) as f32;
                let y2 = (i2 / width) as f32;
                let x2 = (i2 % width) as f32;
                let a = (y2 - y1) / (x2 - x1);
                let b = y1 - a * x1;
                let d = f32::abs(x2 - x1);
                let lower_x = f32::min(x1, x2);
                let upper_x = f32::max(x1, x2);
                let left_x = (lower_x - d).round();
                let right_x = (upper_x + d).round();
                let left_y = (a * left_x + b).round();
                let right_y = (a * right_x + b).round();
                if left_x >= 0.0 && left_x < width as f32 && left_y >= 0.0 && left_y < height as f32 {
                    let left_x = left_x as usize;
                    let left_y = left_y as usize;
                    signals[left_y * width + left_x] = true;
                }
                if right_x >= 0.0 && right_x < width as f32 && right_y >= 0.0 && right_y < height as f32 {
                    let right_x = right_x as usize;
                    let right_y = right_y as usize;
                    signals[right_y * width + right_x] = true;
                }
            }
        }
    }

    // count signals
    let count = signals.iter().filter(|&&x| x).count();

    println!("{}", count);
}
