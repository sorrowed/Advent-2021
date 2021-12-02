use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Movement {
    direction: String,
    count: i64,
}

#[derive(Debug)]
struct ParseMovementError {}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();

        Ok(Movement {
            direction: tokens[0].to_string(),
            count: tokens[1].parse::<i64>().unwrap(),
        })
    }
}

fn parse_movements(name: &str) -> Vec<Movement> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|line| line.unwrap().parse())
        .map(|option| option.unwrap())
        .collect()
}

struct PositionAndAim {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl PositionAndAim {
    pub fn new() -> PositionAndAim {
        PositionAndAim {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn apply_one(&mut self, m: &Movement) {
        match m.direction.as_str() {
            "forward" => self.horizontal += m.count,
            "up" => self.depth -= m.count,
            "down" => self.depth += m.count,
            _ => panic!("Oh noes"),
        }
    }

    pub fn apply_two(&mut self, m: &Movement) {
        match m.direction.as_str() {
            "forward" => {
                self.horizontal += m.count;
                self.depth += self.aim * m.count;
            }
            "up" => self.aim -= m.count,
            "down" => self.aim += m.count,
            _ => panic!("Oh noes"),
        }
    }
}

pub fn part1() {
    let movements = parse_movements("day2/input.txt");

    let mut position = PositionAndAim::new();
    for movement in movements {
        position.apply_one(&movement);
    }

    print!(
        "Day 2 part 1 : Horizontal * Depth : {}\n",
        position.horizontal * position.depth
    );
}

pub fn part2() {
    let movements = parse_movements("day2/input.txt");

    let mut position = PositionAndAim::new();
    for movement in movements {
        position.apply_two(&movement);
    }

    print!(
        "Day 2 part 2 : Horizontal * Depth : {}\n",
        position.horizontal * position.depth
    );
}
