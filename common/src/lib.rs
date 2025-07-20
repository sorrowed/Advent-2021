use std::fs;

pub fn import(name: &str) -> Vec<String> {
    fs::read_to_string(name)
        .unwrap()
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect()
}

#[derive(PartialEq, Eq, Hash)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T>
where
    T: num::Signed,
    T: core::ops::Sub,
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Vector<T> {
        Vector { x: x, y: y, z: z }
    }

    pub fn manhattan(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}
#[cfg(test)]
mod tests {}
