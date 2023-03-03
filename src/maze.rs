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
    data: Vec<Tile>,
}

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
                let coord: Coord = (x, y).into();
                if self.is_edge(coord) && *self.get(coord).unwrap() == Tile::Path {
                    exit.push(coord);
                }
            }
        }
        return exit;
    }

    pub fn neighbours(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        let cardinals: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)]; // V cards - L, U, D, R
        cardinals
            .into_iter()
            .filter_map(move |(dx, dy)| {
                Some(Coord {
                    x: coord.x.checked_add_signed(dx)?,
                    y: coord.y.checked_add_signed(dy)?,
                })
            })
            .filter(|&neighbour| {
                self.is_inbounds(neighbour) && *self.get(neighbour).unwrap() != Tile::Wall
            })
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Maze with width: {} and height: {}\n",
            self.width, self.height
        )?;
        write!(f, " ")?;
        for i in 0..self.width {
            write!(f, "{}", i % 10)?;
        }
        writeln!(f)?;
        for y in 0..self.height {
            write!(f, "{}", y % 10)?;
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
        Ok(())
    }
}
