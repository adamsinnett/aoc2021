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
    let lines: Vec<&str> = input.lines().collect();

    fn oxygen_comp(freq: i32) -> bool {
        freq >= 0
    }
    fn co2_comp(freq: i32) -> bool {
        freq < 0
    }
    let oxygen = i32::from_str_radix(find(lines.clone(), 0, oxygen_comp)[0], 2).unwrap();
    let co2 = i32::from_str_radix(find(lines.clone(), 0, co2_comp)[0], 2).unwrap();

    oxygen * co2
}

fn find<'a>(lines: Vec<&'a str>, idx: usize, comp: fn(i32) -> bool) -> Vec<&'a str> {
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

        if comp(freq) {
            find(
                lines
                    .iter()
                    .filter(|&line| line.bytes().nth(idx) == Some(b'1'))
                    .cloned()
                    .collect(),
                idx + 1,
                comp,
            )
        } else {
            find(
                lines
                    .iter()
                    .filter(|&line| line.bytes().nth(idx) == Some(b'0'))
                    .cloned()
                    .collect(),
                idx + 1,
                comp,
            )
        }
    }
}
