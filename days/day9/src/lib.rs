use common::Vector;

fn part1() {}

fn part2() {}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Add};

    use common::Vector;

    static TEST_INPUT: &[&str] = &[
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    fn create_map(input: &[&str]) -> HashMap<Vector<i32>, u32> {
        input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Vector::new(x as i32, y as i32, 0),
                        c.to_digit(10).expect("Input is not a digit"),
                    )
                })
            })
            .collect()
    }

    #[test]
    fn foo() {
        let map = create_map(TEST_INPUT);
        let local_mins = map
            .iter()
            .filter(|(kc, vc)| {
                // Filter locations where all neighbors have greater 'heights'
                map.iter()
                    .filter(|(kn, _)| kc.manhattan(&kn) == 1)
                    .all(|(_, vn)| vc < &vn)
            })
            .collect::<Vec<_>>();

        assert!(local_mins.len() == 4);
    }
}
