use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Elf {
    number: u32,
    calories: u32,
}

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut elfs: Vec<Elf> = Vec::new();
    let mut elf = Elf {
        number: 1,
        calories: 0,
    };
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line = line.parse::<u32>();

        if let Ok(cals) = line {
            elf.calories += cals;
        } else {
            let next_elf = elf.number + 1;
            elfs.push(elf);
            elf = Elf {
                number: next_elf,
                calories: 0,
            };
        }
    }
    elfs.push(elf);

    let max = elfs.iter().max_by_key(|e| e.calories);
    if let Some(big_elf) = max {
        println!(
            "Max calories are carried by elf: {} and is {}",
            big_elf.number, big_elf.calories
        );
    }

    elfs.sort_by_key(|elf| elf.calories);
    elfs.reverse();
    let sum = elfs[0].calories + elfs[1].calories + elfs[2].calories;
    println!("Top three are carrying {} calories", sum);
}
