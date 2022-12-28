use std::collections::{HashMap, VecDeque};
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

fn is_reachable(program: &[i64], pos: Coord) -> bool {
    let mut state = State::new(program.to_vec());
    state.input.push_back(pos.x);
    state.input.push_back(pos.y);
    while !state.step() {}
    assert_eq!(state.output.len(), 1);
    state.output[0] == 1
}

fn main() {
    let input = File::open("examples/19.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let program: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    let mut best: HashMap<Coord, Coord> = HashMap::new();
    for d in 0 .. {
        for pos in (0 .. d).map(|y| Coord { x: d, y }).chain((0 ..= d).map(|x| Coord { x, y: d })) {
            if !is_reachable(&program, pos) {
                continue;
            }
            let down = Coord { x: pos.x, y: pos.y - 1 };
            let left = Coord { x: pos.x - 1, y: pos.y };
            let x = best.get(&left).cloned().map_or(0, |d| d.x) + 1;
            let y = best.get(&down).cloned().map_or(0, |d| d.y) + 1;
            if x >= 100 && y >= 100 {
                println!("{}", (pos.x - 99) * 10000 + (pos.y - 99));
                return;
            }
            assert!(best.insert(pos, Coord { x, y }).is_none());
        }
    }
}
