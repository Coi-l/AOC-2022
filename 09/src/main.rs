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

    fn move_points(&self, oth: &Point) -> Point {
        let x = self.x - oth.x;
        let y = self.y - oth.y;
        let x = if x.abs() == 2 { x - (x / 2) } else { x };
        let y = if y.abs() == 2 { y - (y / 2) } else { y };
        Point { x, y }
    }
    fn abs_distance(&self, oth: &Point) -> Point {
        let xabs = (self.x - oth.x).abs();
        let yabs = (self.y - oth.y).abs();
        Point { x: xabs, y: yabs }
    }
}
struct Knot {
    pos: Point,
    places: HashSet<Point>,
}
impl Knot {
    fn new() -> Knot {
        let mut knot = Knot {
            pos: Point::new(),
            places: HashSet::new(),
        };
        knot.places.insert(knot.pos);
        knot
    }

    fn do_relative_move(&mut self, pos: &Point) {
        self.pos.x += pos.x;
        self.pos.y += pos.y;
        self.places.insert(self.pos);
    }
}

struct Rope {
    head: Point,
    knots: Vec<Knot>,
}

impl Rope {
    fn new(knots: usize) -> Rope {
        let mut rope = Rope {
            head: Point::new(),
            knots: Vec::new(),
        };
        (0..knots).for_each(|_x| rope.knots.push(Knot::new()));
        rope
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
            //Move Head
            match m {
                Move::Up(_) => self.head.y += 1,
                Move::Down(_) => self.head.y -= 1,
                Move::Left(_) => self.head.x -= 1,
                Move::Right(_) => self.head.x += 1,
            }
            let mut prev_knot_pos = self.head;
            self.knots.iter_mut().for_each(|knot| {
                let abs = prev_knot_pos.abs_distance(&knot.pos);
                if abs.x > 1 || abs.y > 1 {
                    let dist = prev_knot_pos.move_points(&knot.pos);
                    knot.do_relative_move(&dist);
                }
                prev_knot_pos = knot.pos;
            })
        });
        println!();
    }
}

fn trace_rope(rope: &mut Rope, lines: &[String]) -> usize {
    for line in lines.iter() {
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
    rope.knots.last().unwrap().places.len()
}

fn main() {
    let filename = "input";
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    let mut rope = Rope::new(1);
    let visited = trace_rope(&mut rope, &lines);

    let mut rope = Rope::new(9);
    let visited2 = trace_rope(&mut rope, &lines);

    println!("Visited {}", visited);
    println!("Visited {}", visited2);
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
    let mut rope = Rope::new(1);

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
    assert_eq!(rope.knots.last().unwrap().pos.x, 1);
    assert_eq!(rope.knots.last().unwrap().pos.y, 2);
    assert_eq!(rope.knots.last().unwrap().places.len(), 13);
}

// R 5
// U 8
// L 8
// D 3
// R 17
// D 10
// L 25
// U 20
#[test]
fn test_example2() {
    let mut rope = Rope::new(9);

    rope.do_move(Move::Right(5));
    rope.do_move(Move::Up(8));
    rope.do_move(Move::Left(8));
    rope.do_move(Move::Down(3));
    rope.do_move(Move::Right(17));
    rope.do_move(Move::Down(10));
    rope.do_move(Move::Left(25));
    rope.do_move(Move::Up(20));
    assert_eq!(rope.head.x, -11);
    assert_eq!(rope.head.y, 15);
    assert_eq!(rope.knots.last().unwrap().pos.x, -11);
    assert_eq!(rope.knots.last().unwrap().pos.y, 6);
    assert_eq!(rope.knots.last().unwrap().places.len(), 36);
}
