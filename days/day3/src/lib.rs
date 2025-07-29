use common::import;

fn rank(values: &[String], position: usize) -> i32 {
    values
        .iter()
        .fold(0, |a, i| match i.chars().nth(position).unwrap() {
            '0' => a - 1,
            '1' => a + 1,
            _ => panic!("Oh noes!"),
        })
}

fn to_char(rank: i32) -> char {
    if rank > 0 {
        '1'
    } else if rank < 0 {
        '0'
    } else {
        'x'
    }
}

fn invert(value: &str) -> String {
    value.chars().fold("".to_string(), |mut s, c| {
        let inv = if c == '1' {
            '0'
        } else if c == '0' {
            '1'
        } else {
            'x'
        };
        s.push(inv);
        s
    })
}

fn multiply_radix_2(lhs: &str, rhs: &str) -> i64 {
    i64::from_str_radix(rhs, 2).unwrap() * i64::from_str_radix(lhs, 2).unwrap()
}

fn bit_criteria(mut input: Vec<String>, ch: char) -> String {
    let mut position = 0;
    while input.len() > 1 {
        let r = rank(&input, position);

        input.retain(|a| {
            let c = a.chars().nth(position).unwrap();
            (r == 0 && c == ch)
                || (r > 0 && c == ch)
                || (r < 0
                    && c == if ch == '1' {
                        '0'
                    } else if ch == '0' {
                        '1'
                    } else {
                        panic!("Oh noes")
                    })
        });
        position += 1;
    }
    assert!(input.len() == 1);
    input[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let strings = input
            .iter()
            .map(|&s: &&str| s.into())
            .collect::<Vec<_>>();

        assert!(rank(&strings, 0) > 0);
        assert!(rank(&strings, 1) < 0);
        assert!(rank(&strings, 2) > 0);
        assert!(rank(&strings, 3) > 0);
        assert!(rank(&strings, 4) < 0);

        let gamma = (0..5).fold("".to_string(), |mut g, position| {
            g.push(to_char(rank(&strings, position)));
            g
        });
        assert_eq!(gamma, "10110");
        let epsilon = invert(&gamma);
        assert_eq!(epsilon, "01001");

        let power = multiply_radix_2(&gamma, &epsilon);
        assert_eq!(power, 198);

        let o = bit_criteria(strings.clone(), '1');
        assert_eq!(o, "10111");
        let c = bit_criteria(strings.clone(), '0');
        assert_eq!(c, "01010");
    }
}

fn part1() {
    let input = import("days/day3/input.txt");

    let gamma = (0..12).fold("".to_string(), |mut g, position| {
        g.push(to_char(rank(&input, position)));
        g
    });
    let epsilon = invert(&gamma);
    let power = multiply_radix_2(&gamma, &epsilon);

    println!(
        "Day 3 part 1 : Gamma: {} Epsilon: {} Power: {}",
        gamma, epsilon, power
    );
}

fn part2() {
    let input = import("days/day3/input.txt");

    let o = bit_criteria(input.clone(), '1');
    let c = bit_criteria(input.clone(), '0');
    let l = multiply_radix_2(&o, &c);

    println!(
        "Day 3 part 2 : Oxygen rating: {} CO2 scrubber rating: {} Life support rating: {}",
        o, c, l
    );
}

pub fn run() {
    part1();
    part2();
}
