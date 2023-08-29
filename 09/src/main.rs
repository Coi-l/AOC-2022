use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn distance(&self, oth: &Point) -> Point {
        let xabs = (oth.x - self.x).abs();
        let yabs = (oth.y - self.y).abs();
        Point { x: xabs, y: yabs }
        // xabs + yabs
    }
}
struct Tail {
    pos: Point,
    places: HashSet<Point>,
}
impl Tail {
    fn new() -> Tail {
        let mut tail = Tail {
            pos: Point::new(),
            places: HashSet::new(),
        };
        tail.places.insert(tail.pos);
        tail
    }

    fn do_move(&mut self, pos: &Point) {
        println!("Moving tail to: {:?}", pos);
        self.pos.x = pos.x;
        self.pos.y = pos.y;
        self.places.insert(self.pos);
    }
}

struct Rope {
    head: Point,
    tail: Tail,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Point::new(),
            tail: Tail::new(),
        }
    }
    fn do_move(&mut self, move_: Move) {
        let mut expanded = VecDeque::new();
        match move_ {
            Move::Up(s) => (0..s).for_each(|_x| expanded.push_back(Move::Up(0))),
            Move::Down(s) => (0..s).for_each(|_x| expanded.push_back(Move::Down(0))),
            Move::Left(s) => (0..s).for_each(|_x| expanded.push_back(Move::Left(0))),
            Move::Right(s) => (0..s).for_each(|_x| expanded.push_back(Move::Right(0))),
        }

        expanded.iter().for_each(|m| {
            let old_head_position = self.head;
            //Move Head
            match m {
                Move::Up(_) => self.head.y += 1,
                Move::Down(_) => self.head.y -= 1,
                Move::Left(_) => self.head.x -= 1,
                Move::Right(_) => self.head.x += 1,
            }

            let dist = self.head.distance(&self.tail.pos);

            if dist.x > 1 || dist.y > 1 {
                self.tail.do_move(&old_head_position);
            }
        });
    }
}

fn main() {
    let filename = "input";
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut rope = Rope::new();

    for line in reader.lines().flatten() {
        let mut split = line.split_ascii_whitespace();
        let dir = split.next().unwrap();
        let steps = split.next().unwrap().parse::<usize>().unwrap();
        let move_ = match dir {
            "U" => Move::Up(steps),
            "D" => Move::Down(steps),
            "L" => Move::Left(steps),
            "R" => Move::Right(steps),
            _ => Move::Up(0),
        };
        rope.do_move(move_);
    }
    println!("Tail visited {} positions", rope.tail.places.len());
}

#[test]
fn test1() {
    let mut rope = Rope::new();
    rope.do_move(Move::Up(1));
    assert_eq!(rope.head.y, 1);

    rope.do_move(Move::Down(1));
    assert_eq!(rope.head.y, 0);

    rope.do_move(Move::Right(1));
    assert_eq!(rope.head.x, 1);

    rope.do_move(Move::Left(1));
    assert_eq!(rope.head.x, 0);
}

#[test]
fn test2() {
    let mut rope = Rope::new();
    rope.do_move(Move::Up(5));
    assert_eq!(rope.head.y, 5);

    rope.do_move(Move::Down(2));
    assert_eq!(rope.head.y, 3);

    rope.do_move(Move::Right(20));
    assert_eq!(rope.head.x, 20);

    rope.do_move(Move::Left(15));
    assert_eq!(rope.head.x, 5);
}

// R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2
#[test]
fn test_example() {
    let mut rope = Rope::new();

    rope.do_move(Move::Right(4));
    rope.do_move(Move::Up(4));
    rope.do_move(Move::Left(3));
    rope.do_move(Move::Down(1));
    rope.do_move(Move::Right(4));
    rope.do_move(Move::Down(1));
    rope.do_move(Move::Left(5));
    rope.do_move(Move::Right(2));
    assert_eq!(rope.head.x, 2);
    assert_eq!(rope.head.y, 2);
    assert_eq!(rope.tail.pos.x, 1);
    assert_eq!(rope.tail.pos.y, 2);
    assert_eq!(rope.tail.places.len(), 13);
}
