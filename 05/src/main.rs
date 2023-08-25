use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Piles {
    piles: HashMap<usize, VecDeque<char>>,
}
impl Piles {
    fn pile_on(&mut self, pile: usize, c: char) {
        self.piles
            .entry(pile)
            .or_insert(VecDeque::new())
            .push_front(c);
    }

    fn do_move(&mut self, move_: &Move) {
        // println!("Move: {:?}", move_);
        for _x in 0..move_.num {
            let pile = self.piles.get_mut(&move_.from);
            if let Some(pile) = pile {
                let c = pile.pop_back().unwrap();
                // println!("move 1 from {} to {}", move_.from, move_.to);
                if let Some(pile) = self.piles.get_mut(&move_.to) {
                    pile.push_back(c);
                }
            }
        }
    }

    fn do_move_9001(&mut self, move_: &Move) {
        // println!("Move: {:?}", move_);
        let pile = self.piles.get_mut(&move_.from).unwrap();
        let mut tmp_queue: VecDeque<char> = VecDeque::new();
        for _x in 0..move_.num {
            let c = pile.pop_back().unwrap();
            tmp_queue.push_front(c);
        }
        let pile = self.piles.get_mut(&move_.to).unwrap();
        for _x in 0..move_.num {
            let c = tmp_queue.pop_front().unwrap();
            pile.push_back(c);
        }
    }

    fn print_back(&self) {
        for x in 1..=self.piles.len() {
            let c = self.piles.get(&x).unwrap().back().unwrap();
            println!("{}", c);
        }
    }

    fn new() -> Piles {
        Piles {
            piles: HashMap::new(),
        }
    }
}
enum ParseState {
    Crates,
    Nums,
    Moves,
}

fn parse_crates(piles: &mut Piles, line: &String) -> ParseState {
    for (index, c) in line.chars().enumerate() {
        if c.is_ascii_uppercase() {
            let pile = (index / 4) + 1;
            piles.pile_on(pile, c);
        }
        if c.is_numeric() {
            return ParseState::Nums;
        }
    }
    ParseState::Crates
}

fn parse_move(line: &str) -> Result<Move, &'static str> {
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    if let Some(caps) = re.captures(line) {
        let num = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap();
        let to = caps[3].parse::<usize>().unwrap();

        return Ok(Move { num, from, to });
    }
    Err("Failed to parse line")
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut piles = Piles::new();

    let mut parse_state = ParseState::Crates;

    for line in reader.lines().flatten() {
        match parse_state {
            ParseState::Crates => parse_state = parse_crates(&mut piles, &line),
            ParseState::Nums => parse_state = ParseState::Moves,
            ParseState::Moves => {
                let move_ = parse_move(&line).unwrap();
                piles.do_move(&move_);
            }
        }
    }
    println!("Part1:");
    println!("{:?}", piles);
    piles.print_back();
}

fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut piles = Piles::new();

    let mut parse_state = ParseState::Crates;

    for line in reader.lines().flatten() {
        parse_state = match parse_state {
            ParseState::Crates => parse_crates(&mut piles, &line),
            ParseState::Nums => ParseState::Moves,
            ParseState::Moves => {
                let move_ = parse_move(&line).unwrap();
                piles.do_move_9001(&move_);
                ParseState::Moves
            }
        };
    }
    println!("Part2:");
    println!("{:?}", piles);
    piles.print_back();
}
fn main() {
    let filename = "input-example.txt";
    part1(filename);
    part2(filename);
}
