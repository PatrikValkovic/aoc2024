use regex::Regex;

const MAX_WIDTH: i32 = 101;
const MAX_HEIGHT: i32 = 103;
const MAX_STEP: i32 = 100;
const CLUSTER_SIZE_THRESHOLD: usize = 220;

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

fn print_robots(robots: &Vec<Robot>) {
    let mut grid = vec![vec![0; MAX_WIDTH as usize]; MAX_HEIGHT as usize];
    for robot in robots.iter() {
        grid[robot.y as usize][robot.x as usize] += 1;
    }
    for row in grid.iter() {
        for cell in row.iter() {
            print!(
                "{}",
                if *cell == 0 {
                    '.'
                } else {
                    cell.to_string().chars().next().unwrap()
                }
            );
        }
        println!();
    }
    println!();
}

fn is_big_enough_cluster_of_robots(
    robots: &Vec<Robot>,
    grid: &mut Vec<i32>,
    visited: &mut Vec<bool>,
    cluster_threshold: usize,
) -> bool {
    grid.fill(0);
    for robot in robots.iter() {
        grid[(robot.y * MAX_WIDTH + robot.x) as usize] = 1;
    }

    for i in 0..MAX_WIDTH as usize {
        for j in 0..MAX_HEIGHT as usize {
            if grid[j * MAX_WIDTH as usize + i] == 1 {
                // run BFS
                let mut cluster_size = 0;
                visited.fill(false);
                let mut queue = Vec::new();
                queue.push((i, j));

                while !queue.is_empty() {
                    let (x, y) = queue.pop().unwrap();
                    if visited[y * MAX_WIDTH as usize + x] {
                        continue;
                    }
                    if grid[y * MAX_WIDTH as usize + x] == 0 {
                        continue;
                    }
                    cluster_size += 1;
                    visited[y * MAX_WIDTH as usize + x] = true;
                    if cluster_size > cluster_threshold {
                        return true;
                    }
                    if x > 0 {
                        queue.push((x - 1, y));
                    }
                    if x < (MAX_WIDTH - 1) as usize {
                        queue.push((x + 1, y));
                    }
                    if y > 0 {
                        queue.push((x, y - 1));
                    }
                    if y < (MAX_HEIGHT - 1) as usize {
                        queue.push((x, y + 1));
                    }
                }
            }
        }
    }

    return false;
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut robots = Vec::new();
    let lines = input.lines();
    let line_regex = Regex::new(
        r"p=(?<pos_x>[0-9]+),(?<pos_y>[0-9]+) v=(?<velocity_x>-?[0-9]+),(?<velocity_y>-?[0-9]+)",
    )
    .unwrap();
    for line in lines {
        let captures = line_regex.captures(line).unwrap();
        let pos_x = captures
            .name("pos_x")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let pos_y = captures
            .name("pos_y")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let velocity_x = captures
            .name("velocity_x")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let velocity_y = captures
            .name("velocity_y")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        robots.push(Robot::new(pos_x, pos_y, velocity_x, velocity_y));
    }

    // create variables for function in advance
    let mut grid = vec![0; (MAX_WIDTH * MAX_HEIGHT) as usize];
    let mut visited = vec![false; (MAX_WIDTH * MAX_HEIGHT) as usize];
    // do steps
    let mut iteration = 0;
    while true {
        iteration += 1;
        for robot in robots.iter_mut() {
            robot.step(1);
        }
        if is_big_enough_cluster_of_robots(&robots, &mut grid, &mut visited, CLUSTER_SIZE_THRESHOLD) {
            println!("iteration: {}", iteration);
            print_robots(&robots);
            return;
        }
    }
}
