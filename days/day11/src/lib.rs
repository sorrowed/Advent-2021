use std::collections::HashMap;

use common::{enumerate_xy, extends, neighbors, Coordinate};

type Map = HashMap<Coordinate<i64>, Octopus>;

fn step_map(map: &mut Map) -> usize {
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
                    Some(*position)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        result += flashes.len();

        // D: repeat for as long as octopuses keep flashing
        if flashes.is_empty() {
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

fn print_map(map: &Map) {
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
    for _ in 1..=100 {
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
