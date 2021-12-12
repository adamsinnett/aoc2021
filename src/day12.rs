use std::collections::HashMap;

type Node = String;

#[derive(Debug)]
pub struct Maze {
    edges: HashMap<Node, Vec<Node>>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Maze {
    let initial = Maze {
        edges: HashMap::new(),
    };
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .fold(initial, |mut maze, (a, b)| {
            maze.edges
                .entry(a.to_string() as Node)
                .or_insert(Vec::new())
                .push(b.to_string() as Node);
            maze.edges
                .entry(b.to_string() as Node)
                .or_insert(Vec::new())
                .push(a.to_string() as Node);
            maze
        })
}

#[aoc(day12, part1)]
pub fn part1(input: &Maze) -> usize {
    walk_maze(input, "start".to_string(), &mut Vec::new())
}

#[aoc(day12, part2)]
pub fn part2(input: &Maze) -> usize {
    walk_maze_part2(input, "start".to_string(), &mut Vec::new(), false)
}

fn walk_maze(maze: &Maze, node: String, visited: &mut Vec<String>) -> usize {
    maze.edges
        .get(&node)
        .expect(&node)
        .iter()
        .fold(0, |mut acc, node| {
            if node == "end" {
                acc += 1;
            } else if node.to_lowercase() == *node && visited.contains(node) || node == "start" {
                return acc;
            } else {
                visited.push(node.clone());
                acc += walk_maze(maze, node.clone(), visited);
                if visited.pop().unwrap() != *node {
                    panic!("this should work");
                }
            }
            acc
        })
}

fn walk_maze_part2(
    maze: &Maze,
    node: String,
    visited: &mut Vec<String>,
    already_twice: bool,
) -> usize {
    maze.edges
        .get(&node)
        .expect(&node)
        .iter()
        .fold(0, |mut acc, node| {
            let is_lower = node.to_lowercase() == *node;
            let has_visited_twice = if is_lower {
                visited.iter().any(|n| n == node)
            } else {
                false
            };

            if node == "end" {
                acc += 1;
            } else if node == "start" || (has_visited_twice && already_twice) {
                return acc;
            } else {
                visited.push(node.clone());
                acc += walk_maze_part2(
                    maze,
                    node.clone(),
                    visited,
                    already_twice || has_visited_twice,
                );
                if visited.pop().unwrap() != *node {
                    panic!("this should work");
                }
            }
            acc
        })
}
