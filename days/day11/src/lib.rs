use std::{collections::HashMap, thread::sleep, time::Duration};

use common::Coordinate;

pub fn enumerate_xy<F, V>(input: &[&str], f: &F) -> HashMap<Coordinate<i64>, V>
where
    F: Fn(i64, i64, char) -> V,
{
    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Coordinate::new(x as i64, y as i64, 0),
                    f(x as i64, y as i64, c),
                )
            })
        })
        .collect()
}

pub fn extends<I, T>(mut iter: I) -> (Coordinate<T>, Coordinate<T>)
where
    I: Iterator<Item = Coordinate<T>>,
    T: Copy,
    T: std::cmp::PartialOrd,
{
    let mut tl = iter.next().expect("Empty coordinate collection");
    let mut br = tl;

    for i in iter {
        if i.x < tl.x {
            tl.x = i.x
        }
        if i.x > br.x {
            br.x = i.x
        }
        if i.y < tl.y {
            tl.y = i.y
        }
        if i.y > br.y {
            br.y = i.y
        }
    }
    (tl, br)
}

pub fn neighbors(
    location: &Coordinate<i64>,
    extends: &(Coordinate<i64>, Coordinate<i64>),
) -> Vec<Coordinate<i64>> {
    let mut result = vec![];

    for x in -1..=1 {
        for y in -1..=1 {
            let neighbor = location.offset(x, y, 0);

            if neighbor.is_inside(&extends.0, &extends.1) {
                result.push(neighbor);
            }
        }
    }
    result
}

fn step_map(map: &mut HashMap<Coordinate<i64>, Octopus>) -> usize {
    let mut result = 0;

    let extends = extends(map.keys().copied());

    // A: Increase energy level of all octopuses by one
    map.iter_mut()
        .for_each(|(_, octopus)| octopus.energy_level += 1);

    loop {
        // B: Find the ones that should flash (only once per turn)
        let flashes = map
            .iter()
            .filter_map(|(position, octopus)| {
                if !octopus.flashed && octopus.energy_level > 9 {
                    Some(position.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        result += flashes.len();

        // D: repeat for as long as octopuses keep flashing
        if flashes.len() == 0 {
            break;
        }

        // C: Increase energy level for all their neighbors
        for f in flashes.iter() {
            map.entry(*f).and_modify(|o| o.flashed = true);

            let locations: Vec<Coordinate<i64>> = neighbors(f, &extends);

            for l in locations {
                map.entry(l).and_modify(|octopus| {
                    octopus.energy_level += 1;
                });
            }
        }
    }

    map.iter_mut().for_each(|(_, octopus)| {
        if octopus.flashed {
            octopus.energy_level = 0
        }
        octopus.flashed = false
    });

    result
}

fn print_map(map: &HashMap<Coordinate<i64>, Octopus>) {
    let extends = extends(map.keys().copied());
    for y in extends.0.y..=extends.1.y {
        for x in extends.0.x..=extends.1.x {
            print!(
                "{}",
                map.get(&Coordinate::new(x, y, 0)).expect("").energy_level
            );
        }
        println!();
    }
}

struct Octopus {
    energy_level: i32,
    flashed: bool,
}

fn part1() {
    let s = std::fs::read_to_string("days/day11/input.txt").unwrap();
    let input = s.split_terminator('\n').collect::<Vec<_>>();

    let mut map = enumerate_xy(&input, &|_, _, c| Octopus {
        energy_level: c.to_digit(10).expect("Invalid digit") as i32,
        flashed: false,
    });

    let mut flashed = 0;
    for steps in 1..=100 {
        flashed += step_map(&mut map);

        // println!(
        //     "\x1B[2J{}Step {}: flashed {} times",
        //     ansi_control_codes::control_sequences::CUP(0.into(), 0.into()),
        //     steps,
        //     flashed
        // );

        // print_map(&map);
    }

    println!("Day 11 part 1 : Total octopus flashes {}", flashed);
}

fn part2() {
    let s = std::fs::read_to_string("days/day11/input.txt").unwrap();
    let input = s.split_terminator('\n').collect::<Vec<_>>();

    let mut map = enumerate_xy(&input, &|_, _, c| Octopus {
        energy_level: c.to_digit(10).expect("Invalid digit") as i32,
        flashed: false,
    });

    let mut step = 1;

    loop {
        if step_map(&mut map) == 100 {
            break;
        }

        // println!(
        //     "\x1B[2J{}",
        //     ansi_control_codes::control_sequences::CUP(0.into(), 0.into())
        // );

        // print_map(&map);

        step += 1;
    }

    println!("Day 11 part 2 : All octopuses flash at step {}", step);
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::{enumerate_xy, print_map, step_map, Octopus};

    static TEST_INPUT: &[&str] = &[
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    #[test]
    fn part1_test1() {
        let mut map = enumerate_xy(TEST_INPUT, &|_, _, c| Octopus {
            energy_level: c.to_digit(10).expect("Invalid digit") as i32,
            flashed: false,
        });

        assert_eq!(map.len(), 10 * 10);

        let mut flashed = 0;
        for steps in 1..=10 {
            flashed += step_map(&mut map);

            println!(
                "\x1B[2J{}Step {}: flashed {} times",
                ansi_control_codes::control_sequences::CUP(0.into(), 0.into()),
                steps,
                flashed
            );

            print_map(&map);
        }

        assert_eq!(flashed, 204);

        for steps in 11..=100 {
            flashed += step_map(&mut map);

            println!(
                "\x1B[2J{}Step {}: flashed {} times",
                ansi_control_codes::control_sequences::CUP(0.into(), 0.into()),
                steps,
                flashed
            );

            print_map(&map);
        }

        assert_eq!(flashed, 1656);
    }

    #[test]
    fn part2_test1() {
        let mut map = enumerate_xy(TEST_INPUT, &|_, _, c| Octopus {
            energy_level: c.to_digit(10).expect("Invalid digit") as i32,
            flashed: false,
        });

        let mut step = 1;

        loop {
            if step_map(&mut map) == 100 {
                break;
            }
            step += 1;
        }

        assert_eq!(step, 195);
    }
}
