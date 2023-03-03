use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coord {
    // Implement a converter from two usize to a coordinate
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

// Implement both normal and debug printing for coordinates

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
