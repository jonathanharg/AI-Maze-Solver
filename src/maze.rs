use crate::coord::Coord;
use std::fmt;

#[derive(PartialEq)]
pub enum Tile {
    Wall,
    Path,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    data: Vec<Tile>, // Store maze as a continuous list of Tiles
                     // we can calculate (x,y) coords from this list using the width & height
}

// Implement functions for the maze struct
impl Maze {
    pub fn read(text: &str) -> Maze {
        let height = text.trim().lines().count();
        let width = text
            .lines()
            .next()
            .expect("File should not be empty!")
            .replace(' ', "")
            .trim()
            .len();
        let mut data: Vec<Tile> = Vec::new();

        for char in text.chars() {
            let cell = match char {
                '-' => Tile::Path,
                '#' => Tile::Wall,
                '\n' | '\r' | ' ' => continue,
                _ => panic!("Unexpected character '{char}' in maze!"),
            };
            data.push(cell);
        }

        return Maze {
            width,
            height,
            data,
        };
    }

    pub fn get(&self, coord: Coord) -> Option<&Tile> {
        if !self.is_inbounds(coord) {
            return None;
        }
        let index = (coord.y * self.width) + coord.x;
        return Some(&self.data[index]);
    }

    fn is_inbounds(&self, coord: Coord) -> bool {
        return coord.x < self.width && coord.y < self.height;
    }

    pub fn is_edge(&self, coord: Coord) -> bool {
        return coord.x == 0
            || coord.x == self.width - 1
            || coord.y == 0
            || coord.y == self.height - 1;
    }

    pub fn get_exits(&self) -> Vec<Coord> {
        let mut exit: Vec<Coord> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord: Coord = (x, y).into(); // Converts tuple of ints into a coordinate
                if self.is_edge(coord) && *self.get(coord).unwrap() == Tile::Path {
                    exit.push(coord);
                }
            }
        }
        return exit;
    }

    pub fn neighbours(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        // Creates a lazily evaluated iterator over a coordinates neighbours

        // Left, Up, Down, Right
        let cardinals = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        cardinals
            .into_iter() // start by iterating over the cardinal directions
            .filter_map(move |(dx, dy)| {
                // map each direction to a possible coordinate
                Some(Coord {
                    // Signed add so that we cant do 0 - 1 and try get a negative coordinate
                    x: coord.x.checked_add_signed(dx)?,
                    y: coord.y.checked_add_signed(dy)?,
                })
            })
            .filter(|&neighbour| {
                // filter through the coordinates to make sure they are inbound and not walls.
                self.is_inbounds(neighbour) && *self.get(neighbour).unwrap() != Tile::Wall
            })
    }
}

impl fmt::Debug for Maze {
    // Implement a debug print for the Maze struct
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get((x, y).into()).unwrap();
                let c = match cell {
                    Tile::Wall => '#',
                    Tile::Path => '-',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Maze with width: {} and height: {}\n",
            self.width, self.height
        )?;
        return Ok(()); // return Ok if no errors occurred
    }
}
