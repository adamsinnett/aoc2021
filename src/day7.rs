use std::cmp::{max, min};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|d| d.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<usize>) -> usize {
    let middle = input.clone().iter().count() / 2;

    let mut sorted = input.clone();
    sorted.sort();
    let median = sorted.get(middle).unwrap();
    input.iter().map(|d| max(d, median) - min(d, median)).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Vec<usize>) -> i64 {
    let mean: f64 = input.clone().iter().map(|d| *d as i32).sum::<i32>() as f64 / input.len() as f64;
    let rounded = mean as i64;

    input
        .iter()
        .map(|d| *d as i64)
        .map(|d| (d - rounded).abs())
        .map(|n| { 
            n * (n + 1) / 2
        })
        .sum::<i64>()
}
