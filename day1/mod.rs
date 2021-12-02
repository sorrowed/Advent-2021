use crate::common;
use itertools::Itertools;

fn find_descending_1(a: &Vec<i64>) -> i64 {

    let mut deeper = 0;

    for i in 1..a.len() {
        if a[i] > a[i-1] {
            deeper+=1;
        }
    }

    deeper
}

fn find_descending_2(a: &Vec<i64>) -> i64 {

    let mut windows = vec!();
    for (a, b,c) in a.iter().tuple_windows() {
        windows.push(a+b+c);
    }

    find_descending_1(&windows)
}

pub fn part1() {
    let m = find_descending_1(&common::import("day1/input.txt").iter().map(|a|a.parse::<i64>().unwrap()).collect());
    println!("Descending : {}", m);
}

pub fn part2() {
    let m = find_descending_2(&common::import("day1/input.txt").iter().map(|a|a.parse::<i64>().unwrap()).collect());
    println!("Descending : {}", m);
}
