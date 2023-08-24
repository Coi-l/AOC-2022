use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    let mut total_subsets = 0;
    let mut total_disjoints = 0;
    for line in reader.lines().flatten() {
        if let Some(caps) = re.captures(&line) {
            // println!("{}", line);
            let r11 = caps[1].parse::<u32>().unwrap();
            let r12 = caps[2].parse::<u32>().unwrap();
            let r21 = caps[3].parse::<u32>().unwrap();
            let r22 = caps[4].parse::<u32>().unwrap();

            let range1: HashSet<_> = (r11..=r12).collect();
            let range2: HashSet<_> = (r21..=r22).collect();
            if range1.is_subset(&range2) || range2.is_subset(&range1) {
                total_subsets += 1;
                // println!("is subset");
            }

            if !range1.is_disjoint(&range2) {
                total_disjoints += 1;
            }
        }
    }

    println!("Total subsets: {}", total_subsets);
    println!("Total disjoints: {}", total_disjoints);
}
