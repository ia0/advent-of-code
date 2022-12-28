use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Instruction {
    operation: String,
    argument: i64,
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let input: Vec<_> = input.split_whitespace().collect();
        assert_eq!(input.len(), 2);
        let operation = input[0].to_owned();
        let argument = input[1].parse().unwrap();
        Instruction { operation, argument }
    }
}

struct State {
    program: Vec<Instruction>,
    position: i64,
    accumulator: i64,
}

impl State {
    fn new(program: Vec<Instruction>) -> State {
        State { program, position: 0, accumulator: 0 }
    }

    fn step(&mut self) {
        let instruction = &self.program[self.position as usize];
        match &instruction.operation[..] {
            "nop" => self.position += 1,
            "acc" => {
                self.accumulator += instruction.argument;
                self.position += 1;
            }
            "jmp" => self.position += instruction.argument,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = File::open("examples/08.txt").unwrap();
    let mut state = State::new(
        BufReader::new(input).lines().map(|x| Instruction::parse(&x.unwrap())).collect(),
    );
    let mut visited = HashSet::new();
    while visited.insert(state.position) {
        state.step();
    }
    println!("{}", state.accumulator);
}
