use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Reader = std::io::BufReader<std::fs::File>;
#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx,
}
#[derive(Clone, Copy, Debug)]
struct Operation {
    instruction: Instruction,
    arg: isize,
    cycles: usize,
}
impl Operation {
    fn new(instruction: Instruction, arg: isize) -> Operation {
        let cycles = match instruction {
            Instruction::Noop => 1,
            Instruction::Addx => 2,
        };
        Operation {
            instruction,
            arg,
            cycles,
        }
    }
}
type Operations = VecDeque<Operation>;
struct Machine {
    operations: Operations,
    xreg: isize,
    cycle: isize,
    current_operation: Option<Operation>,
}
impl Machine {
    fn new() -> Machine {
        Machine {
            operations: Operations::new(),
            xreg: 1,
            cycle: 1,
            current_operation: None,
        }
    }

    fn signal_strenth(&self) -> isize {
        self.xreg * self.cycle
    }

    fn add_operation(&mut self, op: Operation) {
        self.operations.push_back(op);
        if self.current_operation.is_none() {
            self.current_operation = Some(self.operations.pop_front().unwrap());
        }
    }

    fn run_cycle(&mut self) -> bool {
        if let Some(op) = &mut self.current_operation {
            println!("Cycle: {}, xreg: {}, {:?}", self.cycle, self.xreg, op);
            match op.instruction {
                Instruction::Noop => op.cycles -= 1,
                Instruction::Addx => {
                    match op.cycles {
                        2 => (),
                        1 => {
                            self.xreg += op.arg;
                        }
                        _ => (),
                    }
                    op.cycles -= 1;
                }
            }
            if op.cycles == 0 {
                self.current_operation = self.operations.pop_front();
            }
        }
        self.cycle += 1;
        if self.current_operation.is_some() {
            return true;
        }
        false
    }

    fn run_til_end(&mut self) {
        while self.run_cycle() {}
    }

    fn run_til_cycle(&mut self, cycle: isize) {
        while self.cycle < cycle {
            self.run_cycle();
        }
    }
}

fn parse_instructions(reader: Reader) -> Operations {
    let mut ops = Operations::new();
    for line in reader.lines().flatten() {
        let mut split = line.split_ascii_whitespace();
        if let Some(op) = split.next() {
            match op {
                "addx" => {
                    let arg: isize = split.next().unwrap().parse::<isize>().unwrap();
                    let op = Operation::new(Instruction::Addx, arg);
                    ops.push_back(op);
                }
                "noop" => ops.push_back(Operation::new(Instruction::Noop, 0)),
                _ => (),
            }
        }
    }
    ops
}

fn get_file_reader(filename: &str) -> Reader {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}
fn main() {
    let mut machine = Machine::new();
    let ops = parse_instructions(get_file_reader("input"));
    ops.iter().for_each(|op| machine.add_operation(*op));
    let mut ss = Vec::new();

    machine.run_til_cycle(20);
    ss.push(machine.signal_strenth());

    machine.run_til_cycle(60);
    ss.push(machine.signal_strenth());

    machine.run_til_cycle(100);
    ss.push(machine.signal_strenth());

    machine.run_til_cycle(140);
    ss.push(machine.signal_strenth());

    machine.run_til_cycle(180);
    ss.push(machine.signal_strenth());

    machine.run_til_cycle(220);
    ss.push(machine.signal_strenth());

    let sum: isize = ss.iter().sum();
    println!("Sum signal strengths: {}", sum);
}

#[test]
fn test_noop() {
    let mut machine = Machine::new();
    let op = Operation::new(Instruction::Noop, 0);
    machine.add_operation(op);
    assert_eq!(machine.cycle, 1);
    assert!(!machine.run_cycle());
    assert_eq!(machine.cycle, 2);
    assert_eq!(machine.xreg, 1);
}

#[test]
fn test_addx() {
    let mut machine = Machine::new();
    let op = Operation::new(Instruction::Addx, 1);
    machine.add_operation(op);
    assert_eq!(machine.cycle, 1);
    assert!(machine.run_cycle());
    assert_eq!(machine.cycle, 2);
    assert_eq!(machine.xreg, 1);
    assert!(!machine.run_cycle());
    assert_eq!(machine.cycle, 3);
    assert_eq!(machine.xreg, 2);
}

#[test]
fn test_multiple_ops() {
    let mut machine = Machine::new();
    machine.add_operation(Operation::new(Instruction::Noop, 0));
    machine.add_operation(Operation::new(Instruction::Noop, 0));
    machine.add_operation(Operation::new(Instruction::Addx, 9));
    machine.add_operation(Operation::new(Instruction::Addx, -15));
    machine.run_til_end();
    assert_eq!(machine.xreg, -5);
    assert_eq!(machine.cycle, 7);
}

#[test]
fn test_run_til_cycle() {
    let mut machine = Machine::new();
    machine.add_operation(Operation::new(Instruction::Addx, 9));
    machine.add_operation(Operation::new(Instruction::Addx, 5));
    machine.add_operation(Operation::new(Instruction::Addx, 5));
    machine.add_operation(Operation::new(Instruction::Addx, 15));
    machine.add_operation(Operation::new(Instruction::Addx, 15));
    machine.add_operation(Operation::new(Instruction::Addx, 15));
    machine.run_til_cycle(11);
    assert_eq!(machine.cycle, 11);
    assert_eq!(machine.xreg, 50);
}

#[test]
fn test_example_input() {
    let mut machine = Machine::new();
    let ops = parse_instructions(get_file_reader("input-example"));
    ops.iter().for_each(|op| machine.add_operation(*op));

    machine.run_til_cycle(20);
    assert_eq!(machine.xreg, 21);
    assert_eq!(machine.signal_strenth(), 420);

    machine.run_til_cycle(60);
    assert_eq!(machine.xreg, 19);
    assert_eq!(machine.signal_strenth(), 1140);

    machine.run_til_cycle(100);
    assert_eq!(machine.xreg, 18);
    assert_eq!(machine.signal_strenth(), 1800);

    machine.run_til_cycle(140);
    assert_eq!(machine.xreg, 21);
    assert_eq!(machine.signal_strenth(), 2940);

    machine.run_til_cycle(180);
    assert_eq!(machine.xreg, 16);
    assert_eq!(machine.signal_strenth(), 2880);

    machine.run_til_cycle(220);
    assert_eq!(machine.xreg, 18);
    assert_eq!(machine.signal_strenth(), 3960);
}
