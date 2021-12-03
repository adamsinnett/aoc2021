#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut freqs: Vec<i32> = vec![0; lines[0].len()];

    for line in lines.iter() {
        for i in 0..lines[0].len() {
            if line.bytes().nth(i) == Some(b'1') {
                freqs[i] += 1
            } else {
                freqs[i] -= 1
            }
        }
    }

    let gamma = freqs
        .iter()
        .fold(0, |acc, n| if n >= &0 { (acc * 2) + 1 } else { acc * 2 });

    let epsilon = freqs
        .iter()
        .fold(0, |acc, n| if n < &0 { (acc * 2) + 1 } else { acc * 2 });

    return gamma * epsilon;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let sample =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    let lines: Vec<&str> = input.lines().collect();
    let mut freqs: Vec<i32> = vec![0; lines[0].len()];

    let oxygen = i32::from_str_radix(filter_oxygen(lines.clone(), 0)[0], 2).unwrap();
    let co2 = i32::from_str_radix(filter_co2(lines.clone(), 0)[0], 2).unwrap();

    oxygen * co2
}

fn filter_oxygen<'a>(lines: Vec<&'a str>, idx: usize) -> Vec<&'a str> {
    if lines.len() == 1 {
        lines.iter().cloned().collect()
    } else {
        let freq = lines.iter().fold(0, |accum, line| {
            if line.bytes().nth(idx) == Some(b'1') {
                accum + 1
            } else {
                accum - 1
            }
        });

        if freq >= 0 {
            filter_oxygen(
                lines
                    .iter()
                    .filter(|&line| line.bytes().nth(idx) == Some(b'1'))
                    .cloned()
                    .collect(),
                idx + 1,
            )
        } else {
            filter_oxygen(
                lines
                    .iter()
                    .filter(|&line| line.bytes().nth(idx) == Some(b'0'))
                    .cloned()
                    .collect(),
                idx + 1,
            )
        }
    }
}

// This is ugly and the comparison should be dynamic
fn filter_co2<'a>(lines: Vec<&'a str>, idx: usize) -> Vec<&'a str> {
    if lines.len() == 1 {
        lines.iter().cloned().collect()
    } else {
        let freq = lines.iter().fold(0, |accum, line| {
            if line.bytes().nth(idx) == Some(b'1') {
                accum + 1
            } else {
                accum - 1
            }
        });

        if freq < 0 {
            filter_co2(
                lines
                    .iter()
                    .filter(|&line| line.bytes().nth(idx) == Some(b'1'))
                    .cloned()
                    .collect(),
                idx + 1,
            )
        } else {
            filter_co2(
                lines
                    .iter()
                    .filter(|&line| line.bytes().nth(idx) == Some(b'0'))
                    .cloned()
                    .collect(),
                idx + 1,
            )
        }
    }
}
