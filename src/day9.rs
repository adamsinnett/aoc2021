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

fn find_low_points(input: &Vec<Vec<i32>>) -> Vec<(i32, usize, usize)> {
    let mut low: Vec<(i32, usize, usize)> = Vec::new();
    for x in 0..input.len() {
        for y in 0..input.get(0).expect("vector").len() {
            let cur = get_value(input, x, y);
            if cur < get_value(input, x - 1, y)
                && cur < get_value(input, x, y - 1)
                && cur < get_value(input, x + 1, y)
                && cur < get_value(input, x, y + 1)
            {
                low.push((cur, x, y))
            }
        }
    }
    low
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
pub fn part2(input: &Vec<Vec<i32>>) -> u32 {
    let mut basins: Vec<usize> = find_low_points(input)
        .iter()
        .map(|(_, x, y)| {
            let mut locations: HashSet<(usize, usize)> = HashSet::new();
            find_basin_size(input, &mut locations, *x, *y);
            locations
        })
        .map(|s| s.len())
        .collect();
    basins.sort();
    basins[basins.len() - 3..]
        .iter()
        .fold(1, |acc, basin| acc * basin) as u32
}
