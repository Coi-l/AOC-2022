use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum GameResult {
    Loss = 0,
    Win = 6,
    Draw = 3,
}

struct Game {
    me: Hand,
    opponent: Hand,
}

impl Game {
    fn eval(&self) -> GameResult {
        match self.me {
            Hand::Paper => match self.opponent {
                Hand::Paper => GameResult::Draw,
                Hand::Rock => GameResult::Win,
                Hand::Scissors => GameResult::Loss,
            },
            Hand::Rock => match self.opponent {
                Hand::Paper => GameResult::Loss,
                Hand::Rock => GameResult::Draw,
                Hand::Scissors => GameResult::Win,
            },
            Hand::Scissors => match self.opponent {
                Hand::Paper => GameResult::Win,
                Hand::Rock => GameResult::Loss,
                Hand::Scissors => GameResult::Draw,
            },
        }
    }
    fn score(&self) -> u32 {
        let result = self.eval();
        result as u32 + self.me as u32
    }
}

fn parse_hand(c: char) -> Result<Hand, &'static str> {
    match c {
        'A' => Ok(Hand::Rock),
        'B' => Ok(Hand::Paper),
        'C' => Ok(Hand::Scissors),
        'X' => Ok(Hand::Rock),
        'Y' => Ok(Hand::Paper),
        'Z' => Ok(Hand::Scissors),
        _ => Err("Could not parse hand"),
    }
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for mut line in reader.lines().flatten() {
        let right = line.pop().unwrap();
        line.pop();
        let left = line.pop().unwrap();

        let game = Game {
            me: parse_hand(right).unwrap(),
            opponent: parse_hand(left).unwrap(),
        };
        total_score += game.score();
    }
    println!("Total score (part1): {}", total_score);
}

fn parse_outcome(c: char) -> Result<GameResult, &'static str> {
    match c {
        'X' => Ok(GameResult::Loss),
        'Y' => Ok(GameResult::Draw),
        'Z' => Ok(GameResult::Win),
        _ => Err("Could not parse game result"),
    }
}

fn get_matching_hand(opponent: Hand, desired_result: GameResult) -> Hand {
    match opponent {
        Hand::Paper => match desired_result {
            GameResult::Draw => Hand::Paper,
            GameResult::Loss => Hand::Rock,
            GameResult::Win => Hand::Scissors,
        },
        Hand::Rock => match desired_result {
            GameResult::Draw => Hand::Rock,
            GameResult::Loss => Hand::Scissors,
            GameResult::Win => Hand::Paper,
        },
        Hand::Scissors => match desired_result {
            GameResult::Draw => Hand::Scissors,
            GameResult::Loss => Hand::Paper,
            GameResult::Win => Hand::Rock,
        },
    }
}

fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for mut line in reader.lines().flatten() {
        let outcome = line.pop().unwrap();
        line.pop();
        let opponent = line.pop().unwrap();

        let desired_result = parse_outcome(outcome).unwrap();
        let opponent_played = parse_hand(opponent).unwrap();
        let my_hand = get_matching_hand(opponent_played, desired_result);

        let game = Game {
            me: my_hand,
            opponent: opponent_played,
        };
        // println!("Score: {}", game.score());
        total_score += game.score();
    }
    println!("Total score (part2): {}", total_score);
}

fn main() {
    // let filename = "strategy-guide-example.txt";
    let filename = "strategy-guide.txt";
    part1(filename);
    part2(filename);
}
