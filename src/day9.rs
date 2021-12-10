use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("a number") as i32)
                .collect()
        })
        .collect()
}

fn get_value(input: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    if x >= input.len() || y >= input.get(0).unwrap().len() {
        9
    } else {
        *input.get(x).unwrap().get(y).unwrap()
    }
}

fn is_low(input: &Vec<Vec<i32>>, x: usize, y: usize) -> Option<(i32, usize, usize)> {
    let cur = get_value(input, x, y);
    if cur < get_value(input, x - 1, y)
        && cur < get_value(input, x, y - 1)
        && cur < get_value(input, x + 1, y)
        && cur < get_value(input, x, y + 1)
    {
        Some((cur, x, y))
    } else {
        None
    }
}

fn find_low_points(input: &Vec<Vec<i32>>) -> Vec<(i32, usize, usize)> {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter_map(|(x, y)| is_low(input, x, y))
        .collect()
}

fn find_basin_size(
    input: &Vec<Vec<i32>>,
    locations: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    let curr = get_value(input, x, y);
    if curr == 9 || locations.contains(&(x, y)) {
        return;
    }

    locations.insert((x, y));
    find_basin_size(input, locations, x - 1, y);
    find_basin_size(input, locations, x + 1, y);
    find_basin_size(input, locations, x, y - 1);
    find_basin_size(input, locations, x, y + 1);
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    find_low_points(input).iter().map(|(d, _, _)| d + 1).sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> usize {
    let mut basins: Vec<usize> = find_low_points(input)
        .iter()
        .map(|(_, x, y)| {
            let mut locations: HashSet<(usize, usize)> = HashSet::new();
            find_basin_size(input, &mut locations, *x, *y);
            locations.len()
        })
        .collect();
    basins.sort();
    basins[basins.len() - 3..]
        .iter()
        .fold(1, |acc, basin| acc * basin)
}
