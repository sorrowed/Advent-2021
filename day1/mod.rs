use crate::common;

fn find_descending_1(a: &Vec<i64>) -> usize {
    a.windows(2).filter(|w| w[0] < w[1]).count()
}

fn find_descending_2(a: &Vec<i64>) -> usize {
    find_descending_1(&a.windows(3).map(|w| w.iter().sum()).collect::<Vec<i64>>())
}

pub fn part1() {
    let m = find_descending_1(
        &common::import("day1/input.txt")
            .iter()
            .map(|a| a.parse::<i64>().unwrap())
            .collect(),
    );
    println!("Descending : {}", m);
}

pub fn part2() {
    let m = find_descending_2(
        &common::import("day1/input.txt")
            .iter()
            .map(|a| a.parse::<i64>().unwrap())
            .collect(),
    );
    println!("Descending : {}", m);
}
