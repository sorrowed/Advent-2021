use std::{collections::HashMap, fs};

pub fn import(name: &str) -> Vec<String> {
    fs::read_to_string(name)
        .unwrap()
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Coordinate<T>
where
    T: num::Signed,
    T: Copy,
    T: std::cmp::PartialOrd,
{
    pub fn new(x: T, y: T, z: T) -> Coordinate<T> {
        Coordinate { x, y, z }
    }

    pub fn manhattan(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn offset(&self, x: T, y: T, z: T) -> Self {
        Coordinate {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    pub fn is_inside(&self, top_left: &Coordinate<T>, bottom_right: &Coordinate<T>) -> bool {
        assert!(top_left.x <= bottom_right.x);
        assert!(top_left.y <= bottom_right.y);

        self.x >= top_left.x
            && self.x <= bottom_right.x
            && self.y >= top_left.y
            && self.y <= bottom_right.y
    }
}

/// Enumerate the chars in the input slice and parse these as digits. Return the elements with their coordibates as key.
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

pub fn dfs_all<N, NB, G>(
    current: &N,
    success: &G,
    neighbors: &NB,
    visited: &mut Vec<N>,
    paths: &mut Vec<Vec<N>>,
) where
    N: Clone + Eq + std::hash::Hash,
    NB: Fn(&N, &Vec<N>) -> Vec<N>,
    G: Fn(&N) -> bool,
{
    visited.push(current.clone());

    if success(current) {
        paths.push(visited.clone());
    } else {
        for neighbor in neighbors(current, visited) {
            dfs_all(&neighbor, success, neighbors, visited, paths);
        }
    }
    visited.pop();
}

#[cfg(test)]
mod tests {}
