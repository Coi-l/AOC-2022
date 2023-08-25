use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_marker(line: &str, distinct: usize) -> usize {
    let mut set = HashSet::new();

    for (index, _c) in line.chars().enumerate() {
        set.clear();

        let (_left, right) = line.split_at(index);
        right.chars().take(distinct).for_each(|d| {
            set.insert(d);
        });

        if set.len() == distinct {
            return index + set.len();
        }
    }
    0
}
fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let _r = reader.read_line(&mut line);
    println!("Marker packet start at: {}", find_marker(&line, 4));
    println!("Marker message start at: {}", find_marker(&line, 14));
}

#[test]
fn test1() {
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

#[test]
fn test2() {
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
