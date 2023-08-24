use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return c as u32 - 96;
    }
    if c.is_ascii_uppercase() {
        return c as u32 - 38;
    }

    0
}

fn part1(filename: &str) {
    println!("Begin part1");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines().flatten() {
        let (left, right) = line.split_at(line.len() / 2);
        let items: HashSet<char> = left.chars().collect();
        for c in right.chars() {
            if items.contains(&c) {
                total_score += score(c);
                break;
            }
        }
    }
    println! {"Total score: {}", total_score};
}

#[derive(Debug)]
struct ElfGroup {
    sack1: String,
    sack2: String,
    sack3: String,
}

impl ElfGroup {
    fn find_badge(&self) -> Result<char, String> {
        let set1: HashSet<char> = self.sack1.chars().collect();
        let set2: HashSet<char> = self.sack2.chars().collect();
        let set3: HashSet<char> = self.sack3.chars().collect();
        let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
        let intersection2: Vec<_> = set3.intersection(&intersection).collect();

        if intersection2.len() == 1 {
            Ok(*intersection2[0])
        } else {
            Err(format!("Bad sack, len: {}", intersection2.len()))
        }
    }
}

fn part2(filename: &str) {
    println!("Begin part2");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines().flatten() {
        lines.push(line);
    }

    for i in (0..lines.len()).step_by(3) {
        let eg = ElfGroup {
            sack1: lines[i + 0].clone(),
            sack2: lines[i + 1].clone(),
            sack3: lines[i + 2].clone(),
        };
        // println!("{:?}", eg);
        total_score += score(eg.find_badge().unwrap());
    }
    println!("Total score: {}", total_score);
}

fn main() {
    let filename = "input.txt";
    part1(filename);
    part2(filename);
}

#[test]
fn step() {
    for x in (0..9).step_by(3) {
        println!("{}", x);
    }
}
