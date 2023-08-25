use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_visible(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == trees.len() - 1 || y == trees.len() - 1 {
        return true;
    }

    let my_height = trees[y][x];
    if my_height == 0 {
        return false;
    }
    // println!("Checking {}.{} = {}", x, y, my_height);
    let mut max_height = 0;
    // Check row visibility
    // println!("Check row");
    for n in 0..trees.len() {
        // print!("\tEval {}.{} = {}", n, y, trees[y][n]);
        if n != x {
            max_height = std::cmp::max(max_height, trees[y][n]);
        }
        if n == x || n == trees.len() - 1 {
            if max_height < my_height {
                println!(
                    "{}.{} -> {}.{}: Yes(r) {} {}",
                    x, y, n, y, max_height, my_height
                );
                return true;
            }
            max_height = 0;
            // println!("");
            continue;
        }
        // println!("");
    }

    //Check column visibility
    // println!("Check column");
    max_height = 0;
    for n in 0..trees.len() {
        // print!("\tEval {}.{} = {}", n, y, trees[n][x]);
        if n != y {
            max_height = std::cmp::max(max_height, trees[n][x]);
        }
        if n == y || n == trees.len() - 1 {
            if max_height < my_height {
                // println!(": Yes {} {}", max_height, my_height);
                println!(
                    "{}.{} -> {}.{}: Yes(c) {} {}",
                    x, y, x, n, max_height, my_height
                );
                return true;
            }
            max_height = 0;
            // println!("");
            continue;
        }
        // println!("");
    }

    false
}

fn scenic_score_right(
    trees: &Vec<Vec<u32>>,
    x: isize,
    y: isize,
    my_height: u32,
    distance: usize,
) -> usize {
    if x as usize == trees.len() {
        return 0;
    }
    if trees[y as usize][x as usize] >= my_height {
        return distance + 1;
    }
    scenic_score_right(trees, x + 1, y, my_height, distance) + 1
}

fn scenic_score_left(
    trees: &Vec<Vec<u32>>,
    x: isize,
    y: isize,
    my_height: u32,
    distance: usize,
) -> usize {
    if x == -1 {
        return 0;
    }
    if trees[y as usize][x as usize] >= my_height {
        return distance + 1;
    }
    scenic_score_left(trees, x - 1, y, my_height, distance) + 1
}

fn scenic_score_up(
    trees: &Vec<Vec<u32>>,
    x: isize,
    y: isize,
    my_height: u32,
    distance: usize,
) -> usize {
    if y == -1 {
        return 0;
    }
    if trees[y as usize][x as usize] >= my_height {
        return distance + 1;
    }
    scenic_score_up(trees, x, y - 1, my_height, distance) + 1
}

fn scenic_score_down(
    trees: &Vec<Vec<u32>>,
    x: isize,
    y: isize,
    my_height: u32,
    distance: usize,
) -> usize {
    if y as usize == trees.len() {
        return 0;
    }
    if trees[y as usize][x as usize] >= my_height {
        return distance + 1;
    }
    scenic_score_down(trees, x, y + 1, my_height, distance) + 1
}

fn scenic_score(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let my_height = trees[y][x];
    let xi = isize::try_from(x).unwrap();
    let yi = isize::try_from(y).unwrap();
    let left = scenic_score_left(trees, xi - 1, yi, my_height, 0);
    let right = scenic_score_right(trees, xi + 1, yi, my_height, 0);
    let up = scenic_score_up(trees, xi, yi - 1, my_height, 0);
    let down = scenic_score_down(trees, xi, yi + 1, my_height, 0);
    left * right * up * down
}
fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut trees: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines().flatten() {
        let mut row = Vec::new();
        line.chars().for_each(|c| row.push(c.to_digit(10).unwrap()));
        trees.push(row);
    }

    let mut visible = 0;
    for x in 0..trees.len() {
        for y in 0..trees.len() {
            if is_visible(&trees, x, y) {
                visible += 1;
            }
        }
    }

    println!("Visible: {}", visible);

    let mut scores: Vec<Vec<usize>> = Vec::new();
    for x in 0..trees.len() {
        let mut row = Vec::new();
        for y in 0..trees.len() {
            row.push(scenic_score(&trees, x, y));
        }
        scores.push(row);
    }
    let mut max_score = 0;
    for x in 0..trees.len() {
        // println!();
        for y in 0..trees.len() {
            max_score = std::cmp::max(max_score, scores[y][x]);
            // print!("{}", scores[y][x]);
        }
    }
    println!();
    println!("Max score: {}", max_score);
}
