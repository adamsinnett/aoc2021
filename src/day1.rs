#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|d| d.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> usize {
    input
        .iter()
        .zip(input[1..].iter())
        .filter(|(a, b)| a < b)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> usize {
    part1(
        &(input
            .windows(3)
            .into_iter()
            .map(|x| x.iter().sum())
            .collect()),
    )
}
