mod coord;
use crate::coord::Coord;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Path,
}

struct Maze {
    width: usize,
    height: usize,
    data: Vec<Tile>,
}

impl Maze {
    fn read(text: &str) -> Maze {
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

    fn get(&self, coord: Coord) -> Option<&Tile> {
        if !self.is_inbounds(coord) {
            return None;
        }
        let index = (coord.y * self.width) + coord.x;
        return Some(&self.data[index]);
    }

    fn get_offset(&self, coord: Coord, dx: isize, dy: isize) -> Option<&Tile> {
        let new_y = coord.y.checked_add_signed(dy);
        let new_x = coord.x.checked_add_signed(dx);

        match (new_x, new_y) {
            (Some(x), Some(y)) => self.get((x, y).into()),
            _ => None,
        }
    }

    fn is_inbounds(&self, coord: Coord) -> bool {
        return coord.x < self.width && coord.y < self.height;
    }

    fn is_edge(&self, coord: Coord) -> bool {
        return coord.x == 0
            || coord.x == self.width - 1
            || coord.y == 0
            || coord.y == self.height - 1;
    }

    fn neighbours_iter(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        let cardinals: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        cardinals
            .into_iter()
            .filter_map(move |(dx, dy)| {
                Some(Coord {
                    x: coord.x.checked_add_signed(dx)?,
                    y: coord.y.checked_add_signed(dy)?,
                })
            })
            .filter(|&nb_coord| {
                self.is_inbounds(nb_coord) && *self.get(nb_coord).unwrap() != Tile::Wall
            })
    }

    fn dfs(&mut self, start: Coord, end: Coord) {
        let mut previous: HashMap<Coord, Option<Coord>> = HashMap::new();
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut stack = Vec::new();
        stack.push(start);
        previous.insert(start, None);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            visited.insert(node);
            if node == end {
                break;
            }

            self.neighbours_iter(node).for_each(|n| {
                if !visited.contains(&n) {
                    stack.push(n);
                    previous.insert(n, Some(node));
                }
            });
        }

        let mut path: Vec<Coord> = Vec::new();
        let mut node = Some(end);

        while node.is_some() {
            path.push(node.unwrap());
            node = previous[&node.unwrap()];
        }
        path.reverse();

        println!("Found path of length {}: {:?}", path.len(), path);
    }

    fn parse(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                let coord: Coord = (w, h).into();
                if self.is_edge(coord) && *self.get(coord).unwrap() != Tile::Wall {
                    println!("Found start/end at {}", coord);
                }
                if self.is_junction(coord) {
                    // println!("Found junction at {:?}", coord);
                }
            }
        }
    }

    fn is_junction(&self, coord: Coord) -> bool {
        if *self.get(coord).unwrap() == Tile::Wall {
            return false;
        }

        if self.is_edge(coord) {
            return true;
        }

        let left = self.get_offset(coord, -1, 0);
        let right = self.get_offset(coord, 1, 0);

        if left != right {
            return true;
        }

        let up = self.get_offset(coord, 0, -1);

        if left == up {
            return true;
        }

        let down = self.get_offset(coord, 0, -1);

        return left == down;
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Maze with width: {} and height: {}",
            self.width, self.height
        )?;
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
        Ok(())
    }
}

fn main() {
    let maze_string = include_str!("../maze-Easy.txt");
    let mut grid = Maze::read(maze_string);

    grid.parse();
    println!("{:?}", grid);
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        // grid.dfs((1,0).into(), (1880,999).into());
        grid.dfs((1, 0).into(), (18, 9).into());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
