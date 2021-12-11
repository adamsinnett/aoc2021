use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|d| {
            d.split("")
                .into_iter()
                .filter_map(|d| d.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<Vec<usize>>) -> usize {
    let mut cave = input.clone();
    let mut flashes = 0;
    for _ in 1..=100 {
        let mut flashed: HashSet<(i8, i8)> = HashSet::new();
        let mut exploding: VecDeque<(i8, i8)> = VecDeque::new();
        for x in 0..10 {
            for y in 0..10 {
                explode(&mut cave, (x as i8, y as i8), &mut exploding, &mut flashed);
            }
        }

        while let Some((x, y)) = exploding.pop_front() {
            for ix in (x as i8 - 1)..=(x as i8 + 1) {
                for iy in (y as i8 - 1)..=(y as i8 + 1) {
                    if ix >= 0 && iy >= 0 && ix < 10 && iy < 10 && !flashed.contains(&(ix, iy)) {
                        explode(&mut cave, (ix, iy), &mut exploding, &mut flashed);
                    }
                }
            }
        }
        flashes += flashed.len();
    }
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<Vec<usize>>) -> usize {
    let mut cave = input.clone();
    let mut step = 0;
    loop {
        step += 1;
        let mut flashed: HashSet<(i8, i8)> = HashSet::new();
        let mut exploding: VecDeque<(i8, i8)> = VecDeque::new();
        for x in 0..10 {
            for y in 0..10 {
                explode(&mut cave, (x as i8, y as i8), &mut exploding, &mut flashed);
            }
        }

        while let Some((x, y)) = exploding.pop_front() {
            for ix in (x as i8 - 1)..=(x as i8 + 1) {
                for iy in (y as i8 - 1)..=(y as i8 + 1) {
                    if ix >= 0 && iy >= 0 && ix < 10 && iy < 10 && !flashed.contains(&(ix, iy)) {
                        explode(&mut cave, (ix, iy), &mut exploding, &mut flashed);
                    }
                }
            }
        }
        if flashed.len() == 100 {
            break;
        }
    }
    step
}

fn explode(
    cave: &mut Vec<Vec<usize>>,
    (x, y): (i8, i8),
    exploding: &mut VecDeque<(i8, i8)>,
    flashed: &mut HashSet<(i8, i8)>,
) {
    cave[x as usize][y as usize] += 1;
    if cave[x as usize][y as usize] > 9 {
        cave[x as usize][y as usize] = 0;
        exploding.push_back((x, y));
        flashed.insert((x, y));
    }
}
