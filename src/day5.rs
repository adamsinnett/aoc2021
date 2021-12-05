use regex::Regex;
use std::cmp;
use std::collections::HashMap;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    let re: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|s| {
            let cap = re.captures(s.trim()).expect("regex works yo");
            (
                (
                    cap.get(1)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                    cap.get(2)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                ),
                (
                    cap.get(3)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                    cap.get(4)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                ),
            )
        })
        .collect()
}

fn get_diagonals(input: &Vec<((usize, usize), (usize, usize))>) -> Vec<(usize, usize)> {
    input
        .iter()
        .filter(|((x, y), (a, b))| x != a && y != b)
        .flat_map(|((x, y), (a, b))| {
            let mut coords = Vec::new();
            let mut n = *x;
            let mut m = *y;
            loop {
                coords.push((n, m));
                if n == *a {
                    break;
                }

                if y > b {
                    m -= 1
                } else {
                    m += 1
                }

                if x > a {
                    n -= 1
                } else {
                    n += 1
                }
            }
            coords
        })
        .collect()
}

fn get_lines(input: &Vec<((usize, usize), (usize, usize))>) -> Vec<(usize, usize)> {
    input
        .iter()
        .filter(|((x, y), (a, b))| x == a || y == b)
        .flat_map(|((x, y), (a, b))| {
            let mut coords = Vec::new();
            for n in *cmp::min(x, a)..=*cmp::max(x, a) {
                for m in *cmp::min(y, b)..=*cmp::max(y, b) {
                    coords.push((n, m))
                }
            }
            coords
        })
        .collect()
}

fn get_count_of_dupes(input: Vec<(usize, usize)>) -> usize {
    input
        .iter()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<(usize, usize), usize>, coord| {
                let count = match acc.get(coord) {
                    Some(count) => *count,
                    None => 0,
                };

                acc.insert(*coord, count + 1);
                acc
            },
        )
        .iter()
        .filter(|(_, value)| **value > 1)
        .count()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let points = get_lines(input);
    get_count_of_dupes(points)
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut points = get_lines(input);
    let part2 = get_diagonals(input);
    points.extend(part2);
    get_count_of_dupes(points)
}
