use std::str::FromStr;
use aoc2025::util::read_lines;
use aoc2025::util::get_input_path;

enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError)
        }
    }
}

struct Rotation {
    direction: Direction,
    steps: u32
}

#[derive(Debug)]
struct ParseRotationError;

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, steps_str) = s.split_at(1);
        let direction = dir_str.parse::<Direction>().map_err(|_| ParseRotationError)?;
        let steps = steps_str.parse::<u32>().map_err(|_| ParseRotationError)?;
        Ok(Rotation { direction, steps })
    }
}

impl Rotation {
    fn steps_right(self,) -> i32 {
        match self.direction {
            Direction::Left => -(self.steps as i32),
            Direction::Right => self.steps as i32
        }
    }
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn count_zeros(lines: &Vec<String>) -> i32 {
    lines.into_iter().map(|line| {
        line.parse::<Rotation>().unwrap()
    }).scan(50, |acc, rotation| {
        *acc += rotation.steps_right();
        *acc = modulo(*acc, 100);
        Some(*acc)
    }).filter(|&x| x == 0).count() as i32
}

fn count_tmp_zeros(lines: &Vec<String>) -> i32 {
    lines.into_iter().map(|line| {
        line.parse::<Rotation>().unwrap()
    }).scan(50, |acc, rotation| {
        if rotation.steps == 0 {
            return Some(0);
        }
        let steps_right = rotation.steps_right();
        let initial_position = *acc;
        *acc += steps_right;
        let tmp_zeros =
            if *acc == 0 {
                1
            } else if *acc < 0 {
                -(*acc / 100) + (initial_position != 0) as i32
            } else {
                *acc / 100
            };
        *acc = modulo(*acc, 100);
        Some(tmp_zeros)
    }).sum()
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let zero_count = count_zeros(&lines);
    let tmp_zero_count = count_tmp_zeros(&lines);
    println!("Number of times position is zero: {}", zero_count);
    println!("Number of times position is temporarily zero: {}", tmp_zero_count);
}
