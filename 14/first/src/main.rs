use regex::Regex;

const MAX_WIDTH: i32 = 101;
const MAX_HEIGHT: i32 = 103;
const MAX_STEP: i32 = 100;

struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Robot {
        Robot { x, y, dx, dy }
    }

    fn step(&mut self, steps: i32) {
        for _ in 0..steps / MAX_STEP {
            self.x = (self.x + self.dx * MAX_STEP).rem_euclid(MAX_WIDTH);
            self.y = (self.y + self.dy * MAX_STEP).rem_euclid(MAX_HEIGHT);
        }
        self.x = (self.x + self.dx * (steps % MAX_STEP)).rem_euclid(MAX_WIDTH);
        self.y = (self.y + self.dy * (steps % MAX_STEP)).rem_euclid(MAX_HEIGHT);
    }
}

fn _print_robots(robots: &Vec<Robot>) {
    let mut grid = vec![vec![0; MAX_WIDTH as usize]; MAX_HEIGHT as usize];
    for robot in robots.iter() {
        grid[robot.y as usize][robot.x as usize] += 1;
    }
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", if *cell == 0 { '.' } else { cell.to_string().chars().next().unwrap() });
        }
        println!();
    }
    println!();
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut robots = Vec::new();
    let lines = input.lines();
    let line_regex = Regex::new(r"p=(?<pos_x>[0-9]+),(?<pos_y>[0-9]+) v=(?<velocity_x>-?[0-9]+),(?<velocity_y>-?[0-9]+)").unwrap();
    for line in lines {
        let captures = line_regex.captures(line).unwrap();
        let pos_x = captures.name("pos_x").unwrap().as_str().parse::<i32>().unwrap();
        let pos_y = captures.name("pos_y").unwrap().as_str().parse::<i32>().unwrap();
        let velocity_x = captures.name("velocity_x").unwrap().as_str().parse::<i32>().unwrap();
        let velocity_y = captures.name("velocity_y").unwrap().as_str().parse::<i32>().unwrap();
        robots.push(Robot::new(pos_x, pos_y, velocity_x, velocity_y));
    }

    // _print_robots(&robots);

    // do steps
    for robot in robots.iter_mut() {
        robot.step(100);
    }

    // _print_robots(&robots);

    // calculate quadrants
    let mut quadrants = vec![0; 4];
    for robot in robots.iter() {
        if robot.x < MAX_WIDTH / 2 && robot.y < MAX_HEIGHT / 2 {
            quadrants[0] += 1
        } else if robot.x > MAX_WIDTH / 2 && robot.y < MAX_HEIGHT / 2 {
            quadrants[1] += 1;
        } else if robot.x < MAX_WIDTH / 2 && robot.y > MAX_HEIGHT / 2 {
            quadrants[2] += 1;
        } else if robot.x > MAX_WIDTH / 2 && robot.y > MAX_HEIGHT / 2 {
            quadrants[3] += 1;
        }
    }
    // println!("{:?}", quadrants);

    // multiply quadrants
    let result = quadrants.iter().fold(1, |acc, x| acc * x);
    println!("{}", result);
}
