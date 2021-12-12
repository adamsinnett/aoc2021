use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    value: String,
    is_small: bool,
    is_start: bool,
    is_end: bool,
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node {
            value: s.into(),
            is_small: s.to_ascii_lowercase() == s,
            is_start: s == "start",
            is_end: s == "end",
        })
    }
}

pub type Maze = HashMap<Node, Vec<Node>>;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Maze {
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(from, to)| (from.parse::<Node>().unwrap(), to.parse::<Node>().unwrap()))
        .fold(HashMap::new(), |mut maze, (a, b)| {
            maze.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
            maze.entry(b).or_insert(Vec::new()).push(a);
            maze
        })
}

#[aoc(day12, part1)]
pub fn part1(input: &Maze) -> usize {
    let start = input.keys().into_iter().find(|n| n.is_start).unwrap();
    walk_maze(input, start, &mut Vec::new())
}

#[aoc(day12, part2)]
pub fn part2(input: &Maze) -> usize {
    let start = input.keys().into_iter().find(|n| n.is_start).unwrap();
    walk_maze_part2(input, start, &mut Vec::new(), false)
}

fn walk_maze<'a>(maze: &'a Maze, node: &Node, visited: &mut Vec<&'a Node>) -> usize {
    maze.get(&node).unwrap().iter().fold(0, |mut acc, node| {
        if node.is_end {
            acc += 1;
        } else if node.is_small && visited.contains(&node) || node.is_start {
            return acc;
        } else {
            visited.push(node);
            acc += walk_maze(maze, node, visited);
            visited.pop();
        }
        acc
    })
}

fn walk_maze_part2<'a>(
    maze: &'a Maze,
    node: &Node,
    visited: &mut Vec<&'a Node>,
    already_twice: bool,
) -> usize {
    maze.get(&node).unwrap().iter().fold(0, |mut acc, node| {
        let has_visited_twice = if node.is_small {
            visited.iter().any(|n| n == &node)
        } else {
            false
        };

        if node.is_end {
            acc += 1;
        } else if node.is_start || (has_visited_twice && already_twice) {
            return acc;
        } else {
            visited.push(node);
            acc += walk_maze_part2(maze, node, visited, already_twice || has_visited_twice);
            visited.pop();
        }
        acc
    })
}
