use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

use number_encoding::combinadics::Iter;
use number_encoding::combination;

#[derive(Clone)]
struct State {
    program: Vec<i64>,
    position: usize,
    base: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

#[derive(PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

struct Parameter {
    mode: Mode,
    value: i64,
}

impl State {
    fn new(program: Vec<i64>) -> State {
        State { program, position: 0, base: 0, input: VecDeque::new(), output: VecDeque::new() }
    }

    // Returns whether halted.
    fn step(&mut self) -> bool {
        let opcode = self.program[self.position];
        self.position += 1;
        assert!(opcode >= 0);
        let modes = opcode as usize / 100;
        match opcode % 100 {
            1 => self.i_binop(modes, |x, y| x + y),
            2 => self.i_binop(modes, |x, y| x * y),
            3 => self.i_read(modes),
            4 => self.i_write(modes),
            5 => self.i_jump(modes, false),
            6 => self.i_jump(modes, true),
            7 => self.i_binop(modes, |x, y| (x < y) as i64),
            8 => self.i_binop(modes, |x, y| (x == y) as i64),
            9 => self.i_adjust(modes),
            99 => return true,
            _ => unreachable!(),
        }
        false
    }

    fn needs_input(&self) -> bool {
        self.program[self.position] % 100 == 3 && self.input.is_empty()
    }

    fn param(&mut self, modes: &mut usize) -> Parameter {
        let mode = *modes % 10;
        *modes /= 10;
        let mode = match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => unreachable!(),
        };
        let value = self.program[self.position];
        self.position += 1;
        Parameter { mode, value }
    }

    fn position(&mut self, parameter: Parameter) -> Result<usize, i64> {
        let position = match parameter.mode {
            Mode::Position => parameter.value,
            Mode::Immediate => return Err(parameter.value),
            Mode::Relative => self.base + parameter.value,
        };
        let position = usize::try_from(position).unwrap();
        if self.program.len() <= position {
            self.program.resize(position + 1, 0);
        }
        Ok(position)
    }

    fn read(&mut self, parameter: Parameter) -> i64 {
        match self.position(parameter) {
            Ok(position) => self.program[position],
            Err(value) => value,
        }
    }

    fn write(&mut self, parameter: Parameter, value: i64) {
        let position = self.position(parameter).unwrap();
        self.program[position] = value;
    }

    fn i_binop(&mut self, mut modes: usize, op: impl FnOnce(i64, i64) -> i64) {
        let lhs = self.param(&mut modes);
        let rhs = self.param(&mut modes);
        let out = self.param(&mut modes);
        assert_eq!(modes, 0);
        let lhs = self.read(lhs);
        let rhs = self.read(rhs);
        self.write(out, op(lhs, rhs));
    }

    fn i_read(&mut self, mut modes: usize) {
        let dst = self.param(&mut modes);
        assert_eq!(modes, 0);
        let value = self.input.pop_front().unwrap();
        self.write(dst, value);
    }

    fn i_write(&mut self, mut modes: usize) {
        let src = self.param(&mut modes);
        assert_eq!(modes, 0);
        let src = self.read(src);
        self.output.push_back(src);
    }

    fn i_jump(&mut self, mut modes: usize, is_zero: bool) {
        let cond = self.param(&mut modes);
        let pos = self.param(&mut modes);
        assert_eq!(modes, 0);
        if (self.read(cond) == 0) == is_zero {
            self.position = usize::try_from(self.read(pos)).unwrap();
        }
    }

    fn i_adjust(&mut self, mut modes: usize) {
        let delta = self.param(&mut modes);
        assert_eq!(modes, 0);
        self.base += self.read(delta);
    }
}

fn automate(state: &mut State, mut instructions: impl Iterator<Item = impl AsRef<str>>) {
    loop {
        if state.needs_input() {
            let input = match instructions.next() {
                None => break,
                Some(x) => x,
            };
            println!("{}", input.as_ref());
            state.input.extend(input.as_ref().bytes().map(|x| x as i64));
            state.input.push_back(10);
        }
        if state.step() {
            break;
        }
        if let Some(c) = state.output.pop_front() {
            assert!(state.output.is_empty());
            print!("{}", u8::try_from(c).unwrap() as char);
        }
    }
}

fn brute_force(state: &mut State, items: &[&str]) {
    automate(state, items.iter().map(|item| format!("drop {}", item)));
    let n = items.len();
    for k in 0 ..= n {
        let mut iter = Iter::new(k);
        for _ in 0 .. combination(n, k) {
            let items: Vec<_> = iter.get().iter().map(|&i| items[i]).collect();
            automate(state, items.iter().map(|item| format!("take {}", item)));
            automate(state, ["south"].iter());
            automate(state, items.iter().map(|item| format!("drop {}", item)));
            iter.advance();
        }
    }
}

fn main() {
    let input = File::open("examples/25.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let program: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    let mut state = State::new(program);
    automate(
        &mut state,
        [
            // Hull Breach
            "west",
            // Holodeck
            "take hypercube",
            "west",
            // Stables
            "take space law space brochure",
            "west",
            // Sick Bay
            "north",
            // Gift Wrapping Center
            "take shell",
            "west",
            // Engineering
            "take mug",
            "south",
            // Kitchen
            "take festive hat",
            "north", // Engineering
            "east",  // Gift Wrapping Center
            "south", // Sick Bay
            "west",
            // Navigation
            "east", // Sick Bay
            "east", // Stables
            "east", // Holodeck
            "south",
            // Corridor
            "north", // Holodeck
            "east",  // Hull Breach
            "south",
            // Hallway
            "east",
            // Passages
            "take boulder",
            "west", // Hallway
            "west",
            // Observatory
            "east",  // Hallway
            "north", // Hull Breach
            "east",
            // Crew Quarters
            "north",
            // Warp Drive Maintenance
            "west",
            // Science Lab
            "north",
            // Arcade
            "take whirled peas",
            "west",
            // Storage
            "west",
            // Hot Chocolate Fountain
            "take astronaut ice cream",
            "south",
            // Security Checkpoint
        ]
        .iter(),
    );
    brute_force(
        &mut state,
        &[
            "hypercube",
            "space law space brochure",
            "festive hat",
            "boulder",
            "shell",
            "astronaut ice cream",
            "mug",
            "whirled peas",
        ],
    );
    automate(&mut state, std::io::stdin().lock().lines().map(|x| x.unwrap()));
}
