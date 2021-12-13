use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Fold {
    X,
    Y,
}

#[derive(Debug, Clone)]
pub struct Sonar {
    grid: HashSet<(usize, usize)>,
    inst: VecDeque<(Fold, usize)>,
}

fn translate_x((x, y): &(usize, usize), distance: usize) -> (usize, usize) {
    if *x < distance {
        (*x, *y)
    } else {
        (x - (2 * (x - distance)), *y)
    }
}

fn translate_y((x, y): &(usize, usize), distance: usize) -> (usize, usize) {
    if *y < distance {
        (*x, *y)
    } else {
        (*x, y - (2 * (y - distance)))
    }
}

impl Sonar {
    fn fold(&mut self) -> Option<usize> {
        self.inst
            .pop_front()
            .map(|(fold, distance)| match fold {
                Fold::X => self
                    .grid
                    .iter()
                    .map(|pos| translate_x(pos, distance))
                    .collect(),
                Fold::Y => self
                    .grid
                    .iter()
                    .map(|pos| translate_y(pos, distance))
                    .collect(),
            })
            .map(|updated| {
                self.grid = updated;
                self.count()
            })
    }

    fn count(&self) -> usize {
        self.grid.len()
    }
}

impl fmt::Display for Sonar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row: Vec<&str> = Vec::new();
        for x in 0..6 {
            for y in 0..40 {
                if self.grid.contains(&(y, x)) {
                    row.push("#");
                } else {
                    row.push(".");
                }
            }
            row.push("\n")
        }
        write!(f, "{}", row.join(""))
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Sonar {
    let (str_grid, str_inst) = input.split_once("\n\n").unwrap();
    let grid: HashSet<(usize, usize)> = str_grid
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    let inst: VecDeque<(Fold, usize)> = str_inst
        .lines()
        .map(|l| {
            let replace = l.replace("fold along ", "");
            let (a, b) = replace.split_once("=").unwrap();
            (a.to_string(), b.to_string())
        })
        .map(|(fold, distance)| {
            if fold == "x" {
                (Fold::X, distance.parse::<usize>().unwrap())
            } else {
                (Fold::Y, distance.parse::<usize>().unwrap())
            }
        })
        .collect();

    Sonar { grid, inst }
}

#[aoc(day13, part1)]
pub fn part1(input: &Sonar) -> usize {
    let mut sonar = input.clone();
    sonar.fold();
    sonar.count()
}

#[aoc(day13, part2)]
pub fn part2(input: &Sonar) -> usize {
    let mut sonar = input.clone();
    while sonar.fold().is_some() {}
    println!("{}", sonar);
    0
}
