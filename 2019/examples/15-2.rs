extern crate adventofcode;

use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

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

struct Frame {
    coord: Coord,
    state: State,
}

fn main() {
    let input = File::open("examples/15.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let program: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    let mut todo = VecDeque::new();
    todo.push_back(Frame { coord: Coord::default(), state: State::new(program) });
    let mut wall = HashMap::new();
    wall.insert(Coord::default(), false);
    let mut oxygen = None;
    while let Some(frame) = todo.pop_front() {
        for dir in 1 ..= 4 {
            let coord = match dir {
                1 => Coord { y: frame.coord.y - 1, ..frame.coord },
                2 => Coord { y: frame.coord.y + 1, ..frame.coord },
                3 => Coord { x: frame.coord.x - 1, ..frame.coord },
                4 => Coord { x: frame.coord.x + 1, ..frame.coord },
                _ => unreachable!(),
            };
            if wall.contains_key(&coord) {
                continue;
            }
            let mut state = frame.state.clone();
            state.input.push_back(dir);
            let push = loop {
                assert!(!state.step());
                match state.output.pop_front() {
                    None => continue,
                    Some(0) => break false,
                    Some(1) => break true,
                    Some(2) => {
                        assert!(oxygen.is_none());
                        oxygen = Some(coord);
                        break true;
                    }
                    _ => unreachable!(),
                }
            };
            assert!(wall.insert(coord, !push).is_none());
            if push {
                todo.push_back(Frame { coord, state });
            }
        }
    }
    let mut visited = HashSet::new();
    let mut todo = HashSet::new();
    todo.insert(oxygen.unwrap());
    let mut minutes = 0;
    while !todo.is_empty() {
        let mut next_todo = HashSet::new();
        for coord in todo {
            for dir in 1 ..= 4 {
                let coord = match dir {
                    1 => Coord { y: coord.y - 1, ..coord },
                    2 => Coord { y: coord.y + 1, ..coord },
                    3 => Coord { x: coord.x - 1, ..coord },
                    4 => Coord { x: coord.x + 1, ..coord },
                    _ => unreachable!(),
                };
                if !wall[&coord] && visited.insert(coord) {
                    next_todo.insert(coord);
                }
            }
        }
        todo = next_todo;
        minutes += 1;
    }
    println!("{}", minutes - 1);
}
