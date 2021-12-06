use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|d| d.parse::<i64>().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<i64>) -> i64 {
   let mut numbers: HashMap<i64, i64> = input.iter().fold(HashMap::new(), |mut acc, n| { *acc.entry(*n).or_insert(0) += 1; acc });
   let mut temp: i64 = 0;
   for t in  0..80 {
       let mut n = 8;
        loop {
            if n == 0 {
                
                let zero = numbers.entry(n).or_insert(0).clone();
                *numbers.entry(8).or_insert(0) += zero;
                *numbers.entry(6).or_insert(0) += zero;
                numbers.insert(n.clone(), temp);
                temp = 0;
                break;
            } else {
                let curr = numbers.get(&n).unwrap_or(&0).clone();
                numbers.insert(n.clone(), temp);
                temp = curr.clone();
            }
            n -= 1;

        }
   }

   numbers.values().sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<i64>) -> i64 {
    let mut numbers: HashMap<i64, i64> = input.iter().fold(HashMap::new(), |mut acc, n| { *acc.entry(*n).or_insert(0) += 1; acc });
    let mut temp: i64 = 0;
    for t in  0..256 {
        let mut n = 8;
         loop {
             if n == 0 {
                 let zero = numbers.entry(n).or_insert(0).clone();
                 *numbers.entry(8).or_insert(0) += zero;
                 *numbers.entry(6).or_insert(0) += zero;
                 numbers.insert(n.clone(), temp);
                 temp = 0;
                 break;
             } else {
                 let curr = numbers.get(&n).unwrap_or(&0).clone();
                 numbers.insert(n.clone(), temp);
                 temp = curr.clone();
             }
             n -= 1;
 
         }
    }
 
    numbers.values().sum()
}
