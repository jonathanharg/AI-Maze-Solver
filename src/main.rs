#![allow(clippy::needless_return)]
mod coord;
mod maze;
use crate::coord::Coord;
use clap::{arg, command, value_parser};
use maze::Maze;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{fs, usize};

fn main() {
    // Setup Command Line Arguments
    let user_input = command!()
        .arg(arg!(<maze> "The maze to search").value_parser(value_parser!(PathBuf)))
        .arg(arg!(-s --show "Show the parsed maze").num_args(0))
        .arg(arg!(-a --"a-star" "Search the maze with A*").num_args(0))
        .arg(arg!(-d --dfs "Search the maze with DFS").num_args(0))
        .arg(arg!(-r --"hide-route" "Hide the route taken").num_args(0))
        .get_matches();

    let file = user_input
        .get_one::<PathBuf>("maze")
        .expect("Maze path should be provided!");

    let maze_string = fs::read_to_string(file).expect("Should be able to read the maze file!");
    let maze = Maze::read(&maze_string);

    // Get the flags the user set, defaults to false
    let mut use_a_star = *user_input.get_one::<bool>("a-star").unwrap_or(&false);
    let mut use_dfs = *user_input.get_one::<bool>("dfs").unwrap_or(&false);
    let show_maze = *user_input.get_one::<bool>("show").unwrap_or(&false);
    let hide_route = *user_input.get_one::<bool>("hide-route").unwrap_or(&false);

    if show_maze {
        println!("{:?}", maze);
    }

    // If no search algorithm is specified, use both
    if !use_a_star && !use_dfs {
        use_a_star = true;
        use_dfs = true;
    }

    // Here an "expect" exception is thrown if the start/exit are not found
    let exits = maze.get_exits();
    let start = exits
        .get(0)
        .expect("Maze does not have an entrance or exit!");
    let exit = exits.get(1).expect("Maze does not have an exit!");

    if use_dfs {
        use std::time::Instant;
        let now = Instant::now();
        let (path, explored) = dfs(&maze, *start, *exit);
        let time = now.elapsed();

        println!(
            "DFS: Path len {}, explored {} tiles in {:?}",
            path.len(),
            explored,
            time
        );
        if !hide_route {
            println!("DFS Route: {:?}\n", path)
        }
    }

    if use_a_star {
        use std::time::Instant;
        let now = Instant::now();
        let (path, explored) = a_star(&maze, *start, *exit);
        let time = now.elapsed();
        println!(
            "A*: Path len {}, explored {} tiles in {:?}",
            path.len(),
            explored,
            time
        );
        if !hide_route {
            println!("A* Route: {:?}\n", path)
        }
    }
}

fn dfs(maze: &Maze, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    // Implementation of depth first search, returns the route taken as a vector (list) of Coordinates
    // and also the number of nodes explored as a usize (unsigned pointer-size integer: 32/64 bits)

    // HashMap (Dictionary) to reconstruct the path taken, for each Coordinate we remember an optional
    // coordinate (either Some coordinate it came from or None).
    let mut from: HashMap<Coord, Option<Coord>> = HashMap::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    stack.push(start);
    from.insert(start, None);

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        visited.insert(current);

        if current == end {
            break;
        }

        maze.neighbours(current).for_each(|neighbour| {
            if !visited.contains(&neighbour) {
                stack.push(neighbour);
                from.insert(neighbour, Some(current));
            }
        });
    }

    let path = reconstruct_path(end, from);

    return (path, visited.len());
}

fn reconstruct_path(end: Coord, from: HashMap<Coord, Option<Coord>>) -> Vec<Coord> {
    // Reconstructs the path taken from the 'from' hash map.
    let mut path: Vec<Coord> = Vec::new();
    let mut previous_node = Some(end);

    // While a previous node exists
    while let Some(node) = previous_node {
        path.push(node);
        previous_node = *from
            .get(&node)
            .expect("No path found from start to end, the maze is not solvable!");
    }

    // We have calculated the path backwards from the end, so reverse it.
    path.reverse();

    return path;
}

fn a_star(maze: &Maze, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    // A* Algorithm with a Manhattan Distance heuristic, returns the route taken and
    // the number of nodes explored.

    // The heuristic function, the absolute distance between a & b, i.e. Manhattan Distance
    // h estimates the distance between a & b
    fn h(a: Coord, b: Coord) -> usize {
        return a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
    }

    // Each nodes g score, shortest known path from the start
    let mut g: HashMap<Coord, usize> = HashMap::new();
    g.insert(start, 0);
    // Each nodes f score, the g score (shortest path from start) + heuristic
    let mut f = HashMap::new();
    f.insert(start, h(start, end));
    //Priority Queue (min-heap) of nodes sorted by the lowest f score
    // 'Reverse' is used to sort the queue from smallest f to largest f (since priority queues
    // default largest to smallest)
    let mut queue = PriorityQueue::new();
    queue.push(start, Reverse(h(start, end)));

    let mut from: HashMap<Coord, Option<Coord>> = HashMap::new();
    from.insert(start, None);

    while !queue.is_empty() {
        let current = queue.pop().unwrap().0;

        if current == end {
            break;
        }

        maze.neighbours(current).for_each(|neighbour| {
            // Calculate neighbours new g to be the current g + 1.
            let nbr_g = g.get(&current).unwrap() + 1;

            // If the new g is less than the neighbours known g
            // if no g is known unwrap (default) the value to be the maximum integer
            if nbr_g < *g.get(&neighbour).unwrap_or(&usize::MAX) {
                from.insert(neighbour, Some(current));
                g.insert(neighbour, nbr_g);
                f.insert(neighbour, nbr_g + h(neighbour, end));
                let nbr_f = nbr_g + h(neighbour, end);
                // Add to the queue if the node is not in the queue already
                // If the node is in the queue already, only increase its priority
                queue.push_increase(neighbour, Reverse(nbr_f));
            }
        });
    }
    let visisted = from.len();
    let path = reconstruct_path(end, from);

    return (path, visisted);
}
