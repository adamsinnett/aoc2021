#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|d| d.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> usize {
    let mut clone = input.clone();
    clone.remove(0);

    let zip = input.iter().zip(clone.iter());

    zip.filter(|(a, b)| a < b).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> usize {
    let mut first = input.clone();
    first.remove(0);
    first.remove(1);

    let mut second = input.clone();
    second.remove(0);

    let window: Vec<i32> = second
        .iter()
        .zip(first.iter())
        .map(|(a, b)| a + b)
        .zip(input.iter())
        .map(|(a, b)| a + b)
        .collect();

    part1(&window)
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample1() {}
}
