use std::collections::HashMap;
use std::str::FromStr;

trait Bingo {
    fn turn(&mut self, value: i32) -> ();
    fn won(&self) -> bool;
}

#[derive(Debug, Clone)]
struct Board {
    squares: HashMap<(usize, usize), (bool, i32)>,
    values: HashMap<i32, (usize, usize)>,
    won: bool
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut string: String = String::new();
        for x in 0..5 {
            for y in 0..5 {
                let (found, value) = self.squares.get(&(x, y)).unwrap();
                string.push_str(&format!("{}{:0>2} ", found, value))
            }
            string.push('\n')
        }

        string
    }
}

fn won_x(board: &Board, x: usize, y: usize) -> bool {
    if x > 4 || y > 4 {
        true
    } else {
        let (found, _) = board.squares.get(&(x, y)).unwrap();
        *found && won_x(board, x, y + 1)
    }
}

fn won_y(board: &Board, x: usize, y: usize) -> bool {
    if x > 4 || y > 4 {
        true
    } else {
        let (found, _) = board.squares.get(&(x, y)).unwrap();
        *found && won_y(board, x + 1, y)
    }
}

impl Board {
    fn turn(&mut self, value: i32) -> () {
        if self.values.contains_key(&value) {
            let square: &(usize, usize) = self.values.get(&value).unwrap();
            self.squares.insert(*square, (true, value));
        }
    }

    fn won(&mut self) -> bool {
        for n in 0..5 {
            if won_x(self, n, 0) || won_y(self, 0, n) {
                self.won = true;
                break;
            }
        }
        self.won
    }
}

impl FromStr for Board {
    type Err = ();
    fn from_str(input: &str) -> Result<Board, Self::Err> {
        let mut squares = HashMap::new();
        let mut values = HashMap::new();
        let board_info: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .into_iter()
                    .filter(|d| d.ne(&" ") && d.ne(&""))
                    .map(|d| d.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        for x in 0..board_info.len() {
            for y in 0..board_info[0].len() {
                let value = board_info[x][y];
                squares.insert((x, y), (false, value));
                values.insert(value, (x, y));
            }
        }
        Ok(Board {
            squares: squares,
            values: values,
            won: false
        })
    }
}

#[derive(Debug)]
pub struct Game {
    moves: Vec<i32>,
    boards: Vec<Board>,
}

fn score(board: Board, value: i32) -> i32 {

    let sum: i32 = board
        .squares
        .iter()
        .filter(|(_, (found, _))| !found)
        .map(|(_, (_, value))| value)
        .sum();

    sum * value
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Game {
    input.split("\n\n").into_iter().fold(
        Game {
            moves: Vec::new(),
            boards: Vec::new(),
        },
        |mut game, item| {
            if item.contains("\n") {
                game.boards
                    .push(Board::from_str(item).expect("got a board"))
            } else {
                game.moves.extend(
                    item.split(",")
                        .into_iter()
                        .map(|d| d.parse::<i32>().unwrap()),
                );
            }
            game
        },
    )
}

#[aoc(day4, part1)]
pub fn part1(input: &Game) -> i32 {
    let mut boards = input.boards.clone();
    let mut winner: Option<Board> = None;
    let mut last_value: Option<i32> = None;
    for move_value in input.moves.iter() {
        for board in boards.iter_mut() {
            board.turn(*move_value);
            if board.won() {
                winner = Some(board.clone());
                break;
            }
        }
        if winner.is_some() {
            last_value = Some(*move_value);
            break;
        }
    }

    score(winner.expect("found a winning board"), last_value.expect("found a last value"))
}

#[aoc(day4, part2)]
pub fn part2(input: &Game) -> i32 {
    let mut boards = input.boards.clone();
    let mut winner: Option<Board> = None;
    let mut last_value: Option<i32> = None;
    let num_boards = boards.len();
    let mut num_winners = 0;
    for move_value in input.moves.iter() {
        for board in boards.iter_mut() {
            if board.won {
                continue;
            }
            board.turn(*move_value);

            if board.won() {
                if (num_winners) == num_boards-1 {
                    winner = Some(board.clone());
                    break;
                } else {
                    num_winners+=1
                }
            }
        }
        if winner.is_some() {
            last_value = Some(*move_value);
            break;
        }
    }

    score(winner.expect("found a winning board"), last_value.expect("found a last value"))
}
