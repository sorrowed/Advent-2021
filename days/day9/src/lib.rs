use std::collections::HashMap;

use common::Coordinate;
use itertools::Itertools;

struct LowPoint {
    height: u32,
}

impl LowPoint {
    pub fn risk_level(&self) -> u32 {
        self.height + 1
    }
}

struct HeightMap {
    locations: HashMap<Coordinate<i32>, u32>,
}

impl HeightMap {
    pub fn parse(input: &[&str]) -> HeightMap {
        HeightMap {
            locations: input
                .iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().map(move |(x, c)| {
                        (
                            Coordinate::new(x as i32, y as i32, 0),
                            c.to_digit(10).expect("Input is not a digit"),
                        )
                    })
                })
                .collect(),
        }
    }

    pub fn neighbors(&self, p: &Coordinate<i32>) -> Vec<(Coordinate<i32>, u32)> {
        self.locations
            .iter()
            .filter_map(|(kn, kv)| {
                if p.manhattan(&kn) == 1 {
                    Some((*kn, *kv))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn low_points(&self) -> Vec<(Coordinate<i32>, LowPoint)> {
        self.locations
            .iter()
            .filter(|(kc, vc)| {
                // Filter locations where all neighbors have greater 'heights'
                self.neighbors(kc).iter().all(|(_, vn)| vc < &vn)
            })
            .map(|(location, height)| (*location, LowPoint { height: *height }))
            .collect::<Vec<_>>()
    }

    pub fn determine_basins(
        &self,
        low_points: &Vec<(Coordinate<i32>, LowPoint)>,
    ) -> Vec<HashMap<Coordinate<i32>, u32>> {
        let mut basins = vec![];

        for low_point in low_points.iter() {
            let mut basin = HashMap::from([(low_point.0, low_point.1.height)]);

            self.drill(&mut basin, (low_point.0, low_point.1.height));

            basins.push(basin);
        }
        basins
    }

    fn drill(&self, basin: &mut HashMap<Coordinate<i32>, u32>, current: (Coordinate<i32>, u32)) {
        let neighbors = self.neighbors(&current.0);

        for neighbor in neighbors {
            if neighbor.1 < 9 && neighbor.1 > current.1 {
                if !basin.contains_key(&neighbor.0) {
                    basin.insert(neighbor.0, neighbor.1);

                    //  Do the same for neighbor..
                    self.drill(basin, neighbor);
                }
            }
        }
    }

    pub fn basin_size(&self, basins: &Vec<HashMap<Coordinate<i32>, u32>>) -> usize {
        basins
            .iter()
            .sorted_by_key(|basin| basin.len())
            .rev()
            .take(3)
            .fold(1usize, |acc, basin| acc * basin.len())
    }
}

fn part1() {
    let s = std::fs::read_to_string("days/day9/input.txt").unwrap();

    let map = HeightMap::parse(s.split_terminator('\n').collect::<Vec<_>>().as_slice());

    let low_points = map.low_points();

    let risk_level = low_points.iter().fold(0, |a, e| a + e.1.risk_level());

    println!("Day 9 part 1 : Risk level {}", risk_level);
}

fn part2() {
    let s = std::fs::read_to_string("days/day9/input.txt").unwrap();

    let map = HeightMap::parse(s.split_terminator('\n').collect::<Vec<_>>().as_slice());

    let low_points = map.low_points();

    let basins = map.determine_basins(&low_points);

    let sizes = map.basin_size(&basins);

    assert_eq!(sizes, 1047744);

    println!("Day 9 part 2 : Basin sizes multiplied {}", sizes);
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::HeightMap;

    static TEST_INPUT: &[&str] = &[
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    #[test]
    fn part1_test1() {
        let map = HeightMap::parse(TEST_INPUT);

        let low_points = map.low_points();

        assert_eq!(low_points.len(), 4);

        let risk_level = low_points.iter().fold(0, |a, e| a + e.1.risk_level());

        assert_eq!(risk_level, 15);
    }

    #[test]
    fn part2_test1() {
        let map = HeightMap::parse(TEST_INPUT);

        let low_points = map.low_points();

        let basins = map.determine_basins(&low_points);

        assert_eq!(basins.len(), 4);
        assert_eq!(
            basins.iter().fold(0, |acc, basin| acc + basin.len()),
            3 + 9 + 14 + 9
        );

        let sizes = map.basin_size(&basins);

        assert_eq!(sizes, 1134);
    }
}
