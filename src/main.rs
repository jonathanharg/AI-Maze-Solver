#![allow(clippy::needless_return)]
mod coord;
mod maze;
use clap::{arg, command, value_parser, ArgAction};
use coord::Coord;
use hashbrown::hash_map::DefaultHashBuilder;
use maze::Maze;
use priority_queue::PriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::path::PathBuf;
use std::{fs, usize};

fn main() {
    // Setup Command Line Arguments
    let user_input = command!()
        .arg(
            arg!(<maze> "The maze or mazes to search")
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Append),
        )
        .arg(arg!(-s --show "Show the parsed maze").action(ArgAction::SetTrue))
        .arg(arg!(-a --"a-star" "Search the maze with A*. If no search algorithm is provided, both are used.").action(ArgAction::SetTrue))
        .arg(arg!(-d --dfs "Search the maze with DFS. If no search algorithm is provided, both are used.").action(ArgAction::SetTrue))
        .arg(arg!(-r --"hide-route" "Hide the route taken").action(ArgAction::SetTrue))
        .get_matches();

    // Get the flags the user set, defaults to false
    let mut use_a_star = user_input.get_flag("a-star");
    let mut use_dfs = user_input.get_flag("dfs");
    let show_maze = user_input.get_flag("show");
    let hide_route = user_input.get_flag("hide-route");

    // If no search algorithm is specified, use both
    if !use_a_star && !use_dfs {
        use_a_star = true;
        use_dfs = true;
    }

    // Loop over all given mazes and
    user_input
        .get_many("maze")
        .unwrap()
        .for_each(|file: &PathBuf| {
            let maze_string =
                fs::read_to_string(file).expect("Should be able to read the maze file!");
            let maze = Maze::read(&maze_string);

            println!("Searching {}", file.display());
            if show_maze {
                println!("{:?}", maze);
            }

            let exits = maze.get_exits();
            let start = exits
                .get(0)
                .expect("Maze does not have an entrance or exit!");
            let exit = exits.get(1).expect("Maze does not have an exit!");
            // Here an "expect" exception is thrown if the start/exit are not found

            if use_dfs {
                use std::time::Instant;
                let now = Instant::now();
                let (route, explored) = dfs(&maze, *start, *exit);
                let time = now.elapsed();

                println!(
                    "DFS: Found route with length {}! explored {} nodes in {:?}.",
                    route.len(),
                    explored,
                    time
                );
                if !hide_route {
                    println!("DFS Route: {:?}\n", route)
                }
            }

            if use_a_star {
                use std::time::Instant;
                let now = Instant::now();
                let (route, explored) = a_star(&maze, *start, *exit);
                let time = now.elapsed();
                println!(
                    "A*: Found route with length {}! explored {} nodes in {:?}.",
                    route.len(),
                    explored,
                    time
                );
                if !hide_route {
                    println!("A* Route: {:?}\n", route)
                }
            }
        });
}

fn dfs(maze: &Maze, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    // Implementation of depth first search, returns the route taken as a vector (list) of Coordinates
    // and also the number of nodes explored as a usize (unsigned pointer-size integer: 32/64 bits)

    // HashMap (Dictionary) to reconstruct the route taken. For each Coordinate we remember an optional
    // coordinate (either Some coordinate it came from or None).
    let mut from: FxHashMap<Coord, Option<Coord>> = FxHashMap::default();
    // HashSet which will contain visited nodes.
    let mut visited = FxHashSet::default();
    let mut stack = Vec::new();

    stack.push(start);
    from.insert(start, None);

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        visited.insert(current);

        if current == end {
            // We have found the end of the maze, we can stop now.
            break;
        }

        maze.neighbours(current).for_each(|neighbour| {
            // If we haven't visited this neighbour
            if !visited.contains(&neighbour) {
                stack.push(neighbour);
                from.insert(neighbour, Some(current));
                // Remember where we visited this node from
            }
        });
    }

    let route = reconstruct_route(end, from);

    return (route, visited.len());
}

fn reconstruct_route(end: Coord, from: FxHashMap<Coord, Option<Coord>>) -> Vec<Coord> {
    // Reconstructs the route taken from the 'from' hash map.
    let mut route: Vec<Coord> = Vec::new();
    let mut previous_node = Some(end);

    // While a previous node exists
    while let Some(node) = previous_node {
        route.push(node);
        previous_node = *from
            .get(&node)
            .expect("No route found from start to end, the maze is not solvable!");
    }

    // We have calculated the route backwards from the end, so reverse it.
    route.reverse();

    return route;
}

fn a_star(maze: &Maze, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    // A* Algorithm with a Manhattan Distance heuristic, returns the route taken and
    // the number of nodes explored.

    // The heuristic function, the absolute distance between a & b, i.e. Manhattan Distance
    // h estimates the distance between a & b. Since we can only move horizontally or vertically
    // one step at a time, this is one of the best heuristics.
    fn h(a: Coord, b: Coord) -> usize {
        return a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
    }

    // Each nodes g score, shortest known route from the start
    // Initialize an empty HashMap with a Coordinate key and unsigned int value.
    let mut g: FxHashMap<Coord, usize> = FxHashMap::default();
    g.insert(start, 0); //the distance from the start to the start is always zero.

    // Each nodes f score: the g score (shortest route from start) + heuristic
    let mut f = FxHashMap::default();
    f.insert(start, h(start, end));

    //Priority Queue (min-heap) of nodes sorted by the lowest f score
    // 'Reverse' is used to sort the queue from smallest f to largest f (since priority queues
    // default largest to smallest)
    let mut queue =
        PriorityQueue::<Coord, Reverse<usize>, DefaultHashBuilder>::with_default_hasher();
    queue.push(start, Reverse(h(start, end)));

    let mut from: FxHashMap<Coord, Option<Coord>> = FxHashMap::default();
    from.insert(start, None);

    while !queue.is_empty() {
        // Get the coordinate with the lowest f score from the priority queue
        let current = queue.pop().unwrap().0;

        if current == end {
            // We've found the end, and can stop.
            break;
        }

        maze.neighbours(current).for_each(|neighbour| {
            // Calculate neighbours new g to be the current g + 1.
            let nbr_g = g.get(&current).unwrap() + 1;

            // If the new g is less than the neighbours known g
            // if the g score is None, default the value to be the maximum integer
            if nbr_g < *g.get(&neighbour).unwrap_or(&usize::MAX) {
                // Update the neighbour to be visited by the current node
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
    let visited = from.len();
    let route = reconstruct_route(end, from);

    return (route, visited);
}
