#![allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Path,
    Junction,
}

struct Maze {
    width: usize,
    height: usize,
    data: Vec<Tile>,
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Maze {
    fn parse(text: &str) -> Maze {
        let height = text.trim().lines().count();
        let width = text
            .lines()
            .next()
            .expect("File should not be empty!")
            .replace(" ", "")
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
    fn get(&self, coord: Coord) -> Option<&Tile> {
        if !self.inbounds(coord) {
            return None;
        }
        let index = (coord.y * self.width) + coord.x;
        return Some(&self.data[index]);
    }

    fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        if !self.inbounds(coord) {
            return None;
        }
        let index = (coord.y * self.width) + coord.x;
        return Some(&mut self.data[index]);
    }

    fn inbounds(&self, coord: Coord) -> bool {
        return coord.x < self.width && coord.y < self.height;
    }

    fn is_edge(&self, coord: Coord) -> bool {
        return coord.x == 0
            || coord.x == self.width - 1
            || coord.y == 0
            || coord.y == self.height - 1;
    }

    fn get_offset(&self, coord: Coord, dx: isize, dy: isize) -> Option<&Tile> {
        let new_y = coord.y.checked_add_signed(dy);
        let new_x = coord.x.checked_add_signed(dx);

        match (new_x, new_y) {
            (Some(x), Some(y)) => self.get((x, y).into()),
            _ => None,
        }
    }

    fn neighbours(&self, coord: Coord) -> [Option<&Tile>; 4] {
        let north = self.get_offset(coord, 0, -1);
        let east = self.get_offset(coord, 1, 0);
        let south = self.get_offset(coord, 0, 1);
        let west = self.get_offset(coord, -1, 0);

        return [north, east, south, west];
    }

    fn calculate_junctions(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord: Coord = (x, y).into();
                let tile = self.get(coord).unwrap();

                if *tile == Tile::Wall {
                    continue;
                }

                let is_edge = self.is_edge(coord);

                let wall_neighbours: Vec<bool> = self
                    .neighbours(coord)
                    .into_iter()
                    .flatten()
                    .map(|t| *t == Tile::Wall)
                    .collect();

                let not_two_walls = wall_neighbours.iter().filter(|&&t| t).count() != 2;

                let is_corner = !wall_neighbours.windows(2).all(|t| t[0] != t[1]);

                if is_edge || not_two_walls || is_corner {
                    let tile = self.get_mut(coord).unwrap();
                    *tile = Tile::Junction;
                    continue;
                }
            }
        }
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!(
            "Maze with width: {} and height: {}",
            self.width, self.height
        );
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get((x, y).into()).unwrap();
                let c = match cell {
                    Tile::Wall => '#',
                    Tile::Path => '-',
                    Tile::Junction => '+',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let maze_string = include_str!("../maze-Easy.txt");
    let mut grid = Maze::parse(maze_string);

    grid.calculate_junctions();
    println!("{:?}", grid);
}
