use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Hash, Clone, Debug)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl Eq for Vector {}

impl Vector {
    pub fn new(x: i64, y: i64, z: i64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        f64::hypot((other.y - self.y) as f64, (other.x - self.x) as f64)
    }

    pub fn angle(&self) -> i64 {
        f64::atan2(self.y as f64, self.x as f64).to_degrees() as i64
    }

    pub fn offset(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn manhattan(&self, other: &Vector) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

pub fn import(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        vec.push(line.unwrap());
    }
    vec
}
