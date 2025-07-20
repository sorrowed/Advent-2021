use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct V {
    x: i32,
    y: i32,
}

impl V {
    fn new(x: i32, y: i32) -> V {
        V { x: x, y: y }
    }
}

impl FromStr for V {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(',').collect();

        Ok(V {
            x: tokens[0].parse().unwrap(),
            y: tokens[1].parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Line {
    begin: V,
    end: V,
}

fn step(b: i32, e: i32) -> i32 {
    if b < e {
        1
    } else if b > e {
        -1
    } else {
        0
    }
}
fn between(x: i32, b: i32, e: i32) -> bool {
    if b <= e {
        x >= b && x <= e
    } else {
        x >= e && x <= b
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.begin.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.begin.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn points(&self) -> Vec<V> {
        let mut r = vec![];

        let x_step = step(self.begin.x, self.end.x);
        let y_step = step(self.begin.y, self.end.y);

        let mut x = self.begin.x;
        let mut y = self.begin.y;
        while between(x, self.begin.x, self.end.x) && between(y, self.begin.y, self.end.y) {
            r.push(V { x: x, y: y });
            x += x_step;
            y += y_step;
        }
        r
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" -> ").collect();

        Ok(Line {
            begin: tokens[0].parse().unwrap(),
            end: tokens[1].parse().unwrap(),
        })
    }
}

fn parse_lines(input: &Vec<String>) -> Vec<Line> {
    input.iter().map(|line| line.parse().unwrap()).collect()
}

fn build_map(lines: &Vec<Line>) -> HashMap<V, i32> {
    let mut result = HashMap::<V, i32>::new();

    for line in lines {
        for point in line.points() {
            *result.entry(point).or_insert(0) += 1;
        }
    }
    result
}

fn print_map(top_left: &V, bottom_right: &V, map: &HashMap<V, i32>) {
    for y in top_left.y..=bottom_right.y {
        for x in top_left.x..=bottom_right.x {
            let position = V::new(x, y);

            if let Some(count) = map.get(&position) {
                print!("{}", count);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1() {
    let lines = parse_lines(&common::import("days/day5/input.txt"))
        .into_iter()
        .filter(|line| !line.is_diagonal())
        .collect();

    let map = build_map(&lines);

    print!(
        "Day 5 part 1 : {} positions with 2 or more lines\n",
        map.values().filter(|&c| c >= &2).count()
    );
}

fn part2() {
    let lines = parse_lines(&common::import("days/day5/input.txt"));

    let map = build_map(&lines);

    print!(
        "Day 5 part 2 : {} positions with 2 or more lines\n",
        map.values().filter(|&c| c >= &2).count()
    );
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let lines = parse_lines(&input.iter().map(|s| s.to_string()).collect())
            .into_iter()
            .filter(|line| !line.is_diagonal())
            .collect();

        let map = build_map(&lines);
        // print_map(&V::new(0, 0), &V::new(9, 9), &map);

        assert_eq!(map[&V::new(0, 9)], 2);
        assert_eq!(map[&V::new(1, 9)], 2);
        assert_eq!(map[&V::new(2, 9)], 2);
        assert_eq!(map[&V::new(3, 9)], 1);
    }
}
