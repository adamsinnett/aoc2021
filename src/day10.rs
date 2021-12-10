#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| {
            l.split("")
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn find_bad_syntax(line: &Vec<String>) -> Option<String> {
    let mut stack: Vec<String> = Vec::with_capacity(line.len());
    for s in line.iter() {
        match s.as_ref() {
            "[" | "(" | "{" | "<" => stack.push(s.clone()),
            ")" => {
                if stack.pop().unwrap() != "(" {
                    return Some(s.clone());
                }
            }
            ">" => {
                if stack.pop().unwrap() != "<" {
                    return Some(s.clone());
                }
            }
            "}" => {
                if stack.pop().unwrap() != "{" {
                    return Some(s.clone());
                }
            }
            "]" => {
                if stack.pop().unwrap() != "[" {
                    return Some(s.clone());
                }
            }

            _ => (),
        }
    }
    None
}

fn get_points(bad_char: &str) -> u32 {
    match bad_char {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => panic!("unexpected"),
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<Vec<String>>) -> u32 {
    input
        .iter()
        .filter_map(find_bad_syntax)
        .map(|c| get_points(&c))
        .sum()
}

fn find_incomplete(line: &Vec<String>) -> Option<Vec<String>> {
    let mut stack: Vec<String> = Vec::with_capacity(line.len());
    for s in line.iter() {
        match s.as_ref() {
            "[" | "(" | "{" | "<" => stack.push(s.clone()),
            ")" => {
                if stack.pop().unwrap() != "(" {
                    return None;
                }
            }
            ">" => {
                if stack.pop().unwrap() != "<" {
                    return None;
                }
            }
            "}" => {
                if stack.pop().unwrap() != "{" {
                    return None;
                }
            }
            "]" => {
                if stack.pop().unwrap() != "[" {
                    return None;
                }
            }

            _ => (),
        }
    }
    Some(stack)
}

fn get_points_incomplete(incomplete: &str) -> u64 {
    match incomplete {
        "(" => 1,
        "[" => 2,
        "{" => 3,
        "<" => 4,
        _ => panic!("unexpected"),
    }
}

fn get_points_incomplete_for_stack(stack: Vec<String>) -> u64 {
    stack
        .iter()
        .rev()
        .fold(0, |acc, s| acc * 5 + get_points_incomplete(s))
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<Vec<String>>) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter_map(find_incomplete)
        .map(get_points_incomplete_for_stack)
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
