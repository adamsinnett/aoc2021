use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Day2 {
    dir: Direction,
    amt: i32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Day2> {
    input
        .lines()
        .map(|line| {
            let s: Vec<&str> = line.split(" ").into_iter().collect();
            Day2 {
                dir: Direction::from_str(s[0]).unwrap(),
                amt: s[1].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Day2>) -> i32 {
    let (x, y) = input.iter().fold((0, 0), |(x, y), d| match d.dir {
        Direction::Forward => (x + d.amt, y),
        Direction::Up => (x, y - d.amt),
        Direction::Down => (x, y + d.amt),
    });
    x * y
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Day2>) -> i32 {
    let (x, y, _) = input
        .iter()
        .fold((0, 0, 0), |(h, d, a), inst| match inst.dir {
            Direction::Forward => (h + inst.amt, d + (a * inst.amt), a),
            Direction::Up => (h, d, a - inst.amt),
            Direction::Down => (h, d, a + inst.amt),
        });
    x * y
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2, Day2, Direction};

    #[test]
    fn sample1() {
        assert_eq!(
            input_generator("forward 5")[0],
            Day2 {
                dir: Direction::Forward,
                amt: 5
            }
        );
    }

    #[test]
    fn part1_sample1() {
        let sample = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(part1(&input_generator(sample)), 150);
    }

    #[test]
    fn part2_sample1() {
        let sample = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(part2(&input_generator(sample)), 900);
    }
}
