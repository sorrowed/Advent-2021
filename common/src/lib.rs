use std::fs;

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
        Coordinate { x: x, y: y, z: z }
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
#[cfg(test)]
mod tests {}
