// #![allow(unused_imports, dead_code)]
// #![allow(clippy::needless_return)]
mod coord;
mod maze;
use crate::coord::Coord;
use maze::Maze;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::usize;

fn a_star(maze: &Maze, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    fn h(a: Coord, b: Coord) -> usize {
        return a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
    }
    let mut open_queue = PriorityQueue::new();
    let mut discovered_by: HashMap<Coord, Option<Coord>> = HashMap::new();
    let mut g_score: HashMap<Coord, usize> = HashMap::new();
    let mut f_score = HashMap::new();
    open_queue.push(start, Reverse(h(start, end)));
    discovered_by.insert(start, None);
    g_score.insert(start, 0);
    f_score.insert(start, h(start, end));

    while !open_queue.is_empty() {
        let current = open_queue.pop().unwrap().0;
        if current == end {
            break;
        }

        maze.neighbours(current).for_each(|neighbour| {
            let temp_g_score = g_score.get(&current).unwrap_or(&(usize::MAX - 1)) + 1; // NOTE: Temp d score
            if temp_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                discovered_by.insert(neighbour, Some(current));
                let nbr_f_score = temp_g_score + h(neighbour, end);
                g_score.insert(neighbour, temp_g_score);
                f_score.insert(neighbour, temp_g_score + h(neighbour, end));
                open_queue.push_increase(neighbour, Reverse(nbr_f_score));
            }
        });
    }
    let path = reconstruct_path(end, discovered_by);

    return (path, 0);
}

fn dfs(maze: &Maze, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    let mut from: HashMap<Coord, Option<Coord>> = HashMap::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(start);
    from.insert(start, None);

    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        visited.insert(node);
        if node == end {
            break;
        }

        maze.neighbours(node).for_each(|n| {
            if !visited.contains(&n) {
                stack.push(n);
                from.insert(n, Some(node));
            }
        });
    }

    let path = reconstruct_path(end, from);

    return (path, visited.len());
}

fn reconstruct_path(end: Coord, from: HashMap<Coord, Option<Coord>>) -> Vec<Coord> {
    let mut path: Vec<Coord> = Vec::new();
    let mut backtrack_node = Some(end);

    while backtrack_node.is_some() {
        path.push(backtrack_node.unwrap());
        backtrack_node = *from.get(&backtrack_node.unwrap()).expect("No path found from start to end, the maze may not be solvable!");
    }
    path.reverse();

    return path;
}

fn main() {
    let maze_string = include_str!("../maze-Debug2.txt");
    let maze = Maze::read(maze_string);

    // println!("{:?}", maze);
    let exits = maze.get_exits();
    let start = exits.get(0).expect("Maze does not have an enterance or exit!");
    let exit = exits.get(1).expect("Maze does not have an exit!");

    let a_star_start = Instant::now();
    let (a_star_path, a_star_explored) = a_star(&maze, *start, *exit);
    let a_star_time = a_star_start.elapsed();

    let dfs_start = Instant::now();
    let (dfs_path, dfs_explored) = dfs(&maze, *start, *exit);
    let dfs_time = dfs_start.elapsed();

    println!("DFS: Path len {}, explored {} in {:?}", dfs_path.len(), dfs_explored, dfs_time);
    println!("A*: Path len {}, explored {} in {:?}", a_star_path.len(), a_star_explored, a_star_time);
}
