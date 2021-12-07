#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|d| d.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<i64>) -> i64 {
    let median = input.clone().iter().count() / 2;

    let mut sorted = input.clone();
    sorted.sort();
    let n = sorted.get(median).unwrap();
    input.iter().map(|d| (d - n).abs()).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Vec<i64>) -> i64 {
    let mean: f64 = input.clone().iter().sum::<i64>() as f64 / input.len() as f64;

    input
        .iter()
        .map(|d| (d - mean as i64).abs())
        .map(|n| n * (n + 1) / 2)
        .sum()
}
