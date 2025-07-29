fn find_descending_1(a: &[i64]) -> usize {
    a.windows(2).filter(|w| w[0] < w[1]).count()
}

fn find_descending_2(a: &[i64]) -> usize {
    find_descending_1(&a.windows(3).map(|w| w.iter().sum()).collect::<Vec<i64>>())
}

fn part1() {
    let m = find_descending_1(
        &common::import("days/day1/input.txt")
            .iter()
            .map(|a| a.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
    );
    println!("Day 1 part 1 : Descending : {}", m);
}

fn part2() {
    let m = find_descending_2(
        &common::import("days/day1/input.txt")
            .iter()
            .map(|a| a.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
    );
    println!("Day 1 part 2 : Descending : {}", m);
}

pub fn run() {
    part1();
    part2();
}
