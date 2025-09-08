use std::{collections::HashMap, str::FromStr};

use common::Coordinate;
use regex::Regex;

struct Map {
    fields: HashMap<Coordinate<i32>, i32>,
}

impl Map {
    pub fn parse(dots: &[&str]) -> Map {
        let mut fields = HashMap::new();

        for line in dots {
            let mut s = line.split(",");
            fields.insert(
                Coordinate::new(
                    s.next()
                        .expect("Error parsing field : x")
                        .parse::<i32>()
                        .unwrap(),
                    s.next()
                        .expect("Error parsing field : y")
                        .parse::<i32>()
                        .unwrap(),
                    0,
                ),
                1,
            );
        }

        Map { fields }
    }

    pub fn fold(&mut self, fold: &Fold) {
        match fold.axis {
            Axis::X => self.fold_vertical(fold.offset),
            Axis::Y => self.fold_horizontal(fold.offset),
        }
    }

    fn fold_horizontal(&mut self, offset: i32) {
        let (x_min, x_max, y_min, _) = self.extends();

        for y in y_min..offset {
            for x in x_min..=x_max {
                let folded = self
                    .fields
                    .entry(Coordinate::new(x, 2 * offset - y, 0))
                    .or_insert(0);

                *self.fields.entry(Coordinate::new(x, y, 0)).or_insert(0) |= *folded;
            }
        }

        self.fields.retain(|e, _| e.y < offset);
    }

    fn fold_vertical(&mut self, offset: i32) {
        let (x_min, _, y_min, y_max) = self.extends();

        for y in y_min..=y_max {
            for x in x_min..offset {
                let folded = self
                    .fields
                    .entry(Coordinate::new(2 * offset - x, y, 0))
                    .or_insert(0);

                *self.fields.entry(Coordinate::new(x, y, 0)).or_insert(0) |= *folded;
            }
        }

        self.fields.retain(|e, _| e.x < offset);
    }

    fn extends(&self) -> (i32, i32, i32, i32) {
        let x_min = self.fields.keys().min_by_key(|c| c.x).expect("Eeps").x;
        let x_max = self.fields.keys().max_by_key(|c| c.x).expect("Eeps").x;
        let y_min = self.fields.keys().min_by_key(|c| c.y).expect("Eeps").y;
        let y_max = self.fields.keys().max_by_key(|c| c.y).expect("Eeps").y;
        (x_min, x_max, y_min, y_max)
    }

    fn print(&self) {
        let (x_min, x_max, y_min, y_max) = self.extends();

        println!();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let c = self
                    .fields
                    .get(&Coordinate { x, y, z: 0 })
                    .or(Some(&0))
                    .expect("Eeps");

                print!("{}", if c == &1 { '#' } else { '.' });
            }
            println!();
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, PartialEq)]
struct AxisParseError {}

impl FromStr for Axis {
    type Err = AxisParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(AxisParseError {}),
        }
    }
}

struct Fold {
    axis: Axis,
    offset: i32,
}

fn parse_folds(folds: &[&str]) -> Vec<Fold> {
    let mut result = vec![];
    let re = Regex::new(r"fold along (?<axis>x|y)=(?<offset>\d+)").unwrap();

    for fold in folds {
        if let Some(results) = re.captures(fold) {
            let axis = &results["axis"]
                .parse::<Axis>()
                .unwrap_or_else(|_| panic!("Invalid fold axis {}", &results["axis"].to_string()));

            let offset = &results["offset"]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Invalid fold offset {}", &results["offset"].to_string()));

            result.push(Fold {
                axis: axis.clone(),
                offset: *offset,
            });
        }
    }

    result
}

fn parse_dots_and_folds<'a>(input: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) {
    let end_of_dots = input
        .iter()
        .enumerate()
        .find(|(_, &s)| s.is_empty())
        .expect("No empty line found in instructions")
        .0;
    let (dots, folds) = input.split_at(end_of_dots);
    let folds: &[&str] = folds
        .split_first()
        .expect("Failed to remove first empty line from folds")
        .1;
    (dots, folds)
}

fn part1() {
    let input = common::import("days/day13/input.txt");
    let b = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let (dots, folds) = parse_dots_and_folds(&b);

    let mut map = Map::parse(dots);
    let folds = parse_folds(folds);

    map.fold(&folds[0]);

    println!(
        "Day 13 part 1 : Visible dots {}",
        map.fields.values().filter(|&v| v != &0).count()
    );
}

fn part2() {
    let input = common::import("days/day13/input.txt");
    let b = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let (dots, folds) = parse_dots_and_folds(&b);

    let mut map = Map::parse(dots);
    let folds = parse_folds(folds);

    for fold in folds {
        map.fold(&fold);
    }

    map.print();

    println!(
        "Day 13 part 2 : Visible dots {}",
        map.fields.values().filter(|&v| v != &0).count()
    );
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::{parse_dots_and_folds, parse_folds, Axis, Map};

    static TEST_INPUT1: &[&str] = &[
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    #[test]
    fn part1_test1() {
        let (dots, folds) = parse_dots_and_folds(TEST_INPUT1);

        assert_eq!(dots.len(), 18);
        assert_eq!(folds.len(), 2);

        let mut map = Map::parse(dots);

        let folds = parse_folds(folds);
        assert_eq!(folds.len(), 2);
        assert_eq!(folds[0].axis, Axis::Y);
        assert_eq!(folds[0].offset, 7);
        assert_eq!(folds[1].axis, Axis::X);
        assert_eq!(folds[1].offset, 5);

        map.print();
        map.fold(&folds[0]);
        map.print();

        assert_eq!(map.fields.values().filter(|&v| v != &0).count(), 17);

        map.fold(&folds[1]);
        map.print();

        assert_eq!(map.fields.len(), 5 * 7);
    }
}
