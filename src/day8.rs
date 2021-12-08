use std::collections::HashMap;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<String>>> {
    input
        .lines()
        .map(|s| {
            s.split("|")
                .into_iter()
                .map(|s| {
                    s.trim()
                        .split_whitespace()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &Vec<Vec<Vec<String>>>) -> usize {
    let second: Vec<Vec<String>> = input.iter().map(|s| s[1].clone()).collect();
    second
        .iter()
        .map(|s| {
            s.iter()
                .filter(|w| w.len() == 2 || w.len() == 3 || w.len() == 4 || w.len() == 7)
                .count()
        })
        .sum()
}

fn decode(decoder: HashMap<String, String>, input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|c| {
            decoder
                .clone()
                .into_iter()
                .find(|(k, _)| k.len() == c.len() && contains(k, c))
                .unwrap()
                .1
        })
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn contains(parent: &str, child: &str) -> bool {
    child.chars().all(|c| parent.contains(c))
}

fn get_decoder(input: &Vec<String>) -> HashMap<String, String> {
    let mut decoder = HashMap::new();
    let one = input.iter().find(|s| s.len() == 2).unwrap();
    let four = input.iter().find(|s| s.len() == 4).unwrap();
    let seven = input.iter().find(|s| s.len() == 3).unwrap();
    let eight = input.iter().find(|s| s.len() == 7).unwrap();
    let three = input
        .iter()
        .find(|s| s.len() == 5 && contains(s, seven))
        .unwrap();
    let nine = input
        .iter()
        .find(|s| s.len() == 6 && contains(s, three) && contains(s, four))
        .unwrap();
    let zero = input
        .iter()
        .find(|s| s.len() == 6 && contains(s, seven) && !contains(s, three))
        .unwrap();
    let six = input
        .iter()
        .find(|s| s.len() == 6 && !contains(s, nine) && !contains(s, zero))
        .unwrap();
    let five = input
        .iter()
        .find(|s| s.len() == 5 && contains(six, s))
        .unwrap();
    let two = input
        .iter()
        .find(|s| s.len() == 5 && !contains(s, five) && !contains(s, seven))
        .unwrap();

    decoder.insert(zero.clone(), "0".to_string());
    decoder.insert(one.clone(), "1".to_string());
    decoder.insert(two.clone(), "2".to_string());
    decoder.insert(three.clone(), "3".to_string());
    decoder.insert(four.clone(), "4".to_string());
    decoder.insert(five.clone(), "5".to_string());
    decoder.insert(six.clone(), "6".to_string());
    decoder.insert(seven.clone(), "7".to_string());
    decoder.insert(eight.clone(), "8".to_string());
    decoder.insert(nine.clone(), "9".to_string());

    decoder
}

#[aoc(day8, part2)]
pub fn part2(input: &Vec<Vec<Vec<String>>>) -> usize {
    input
        .iter()
        .map(|v| {
            let decoder = get_decoder(&v[0]);
            let decoded = decode(decoder, &v[1]);
            decoded
        })
        .sum()
}
