use std::collections::HashMap;

fn generation(v: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut r = HashMap::<i64, i64>::new();

    let s = v.get(&0).unwrap();

    for a in (0..8).rev() {
        r.insert(a, *v.get(&(a + 1)).unwrap());
    }

    r.entry(6).and_modify(|n| *n += s);
    *r.entry(8).or_insert(0) += s;

    r
}

fn make_map(fish: &Vec<i64>) -> HashMap<i64, i64> {
    let mut r = HashMap::<i64, i64>::new();

    for n in 0i64..=8i64 {
        r.insert(n, 0);
    }
    for f in fish {
        r.entry(*f).and_modify(|n| *n += 1);
    }
    r
}

fn input() -> Vec<i64> {
    vec![
        1, 1, 1, 2, 1, 5, 1, 1, 2, 1, 4, 1, 4, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 5, 1, 3,
        1, 2, 1, 1, 1, 2, 1, 1, 1, 4, 1, 1, 3, 1, 5, 1, 1, 1, 1, 3, 5, 5, 2, 1, 1, 1, 2, 1, 1, 1,
        1, 1, 1, 1, 1, 5, 4, 1, 1, 1, 1, 1, 3, 1, 1, 2, 4, 4, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 5,
        1, 3, 1, 5, 1, 2, 1, 1, 5, 1, 1, 1, 5, 3, 3, 1, 4, 1, 3, 1, 3, 1, 1, 1, 1, 3, 1, 4, 1, 1,
        1, 1, 1, 2, 1, 1, 1, 4, 2, 1, 1, 5, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
        5, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 4, 1, 2, 1, 3, 2, 1, 1, 2, 1, 1, 1, 1, 4, 1, 1, 1, 1,
        4, 1, 1, 1, 1, 1, 2, 1, 1, 4, 1, 1, 1, 5, 3, 2, 2, 1, 1, 3, 1, 5, 1, 5, 1, 1, 1, 1, 1, 5,
        1, 4, 1, 2, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 1, 4, 3, 1, 4, 1, 3, 2,
        1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 5, 1, 1, 1, 1, 2, 1, 1, 1, 3, 5, 1,
        1, 1, 1, 5, 1, 1, 2, 1, 2, 4, 2, 2, 1, 1, 1, 5, 2, 1, 1, 5, 1, 1, 1, 1, 5, 1, 1, 1, 2, 1,
    ]
}

fn part1() {
    let mut fish = make_map(&input());

    for _ in 0..80 {
        fish = generation(&mut fish);
    }

    print!(
        "Day 6 part 1 : After 80 days there are {} lanterfish\n",
        fish.values().fold(0, |a, &v| a + v)
    );
}

fn part2() {
    let mut fish = make_map(&input());

    for _ in 0..256 {
        fish = generation(&mut fish);
    }

    print!(
        "Day 6 part 2 : After 256 days there are {} lanterfish\n",
        fish.values().fold(0, |a, &v| a + v)
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
        let mut fish = make_map(&vec![3, 4, 3, 1, 2]);

        for _ in 0..80 {
            //print!("{} --> {:?}\n", d, fish);
            fish = generation(&fish);
        }
        assert_eq!(fish.values().fold(0, |a, &v| a + v), 5934);
    }
}
