use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
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

impl Instruction {
    fn mutate(&mut self) -> bool {
        match &self.operation[..] {
            "nop" => self.operation = "jmp".to_string(),
            "jmp" => self.operation = "nop".to_string(),
            "acc" => return false,
            _ => unreachable!(),
        }
        true
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

    fn reset(&mut self, position: i64) {
        self.position = position;
        self.accumulator = 0;
    }

    fn done(&self) -> bool {
        self.position == self.program.len() as i64
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
    let mut todo = Vec::new();
    todo.push((0, None));
    while let Some((position, mutated)) = todo.pop() {
        if !visited.insert((position, mutated)) {
            continue;
        }
        state.reset(position);
        if state.done() {
            assert!(state.program[mutated.unwrap() as usize].mutate());
            break;
        }
        if mutated.is_none() && state.program[position as usize].mutate() {
            state.step();
            todo.push((state.position, Some(position)));
            assert!(state.program[position as usize].mutate());
            state.reset(position);
        }
        state.step();
        todo.push((state.position, mutated));
    }
    state.reset(0);
    while !state.done() {
        state.step();
    }
    println!("{}", state.accumulator);
}
