use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    let input = File::open("examples/23.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let program: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    const N: usize = 50;
    let mut states: Vec<_> = (0 .. N)
        .map(|i| {
            let mut state = State::new(program.clone());
            state.input.push_back(i as i64);
            state
        })
        .collect();
    let mut nat: Option<(i64, i64)> = None;
    let mut idle = vec![-1; N];
    let mut last_y = None;
    loop {
        for state in &mut states {
            if state.input.is_empty() {
                state.input.push_back(-1);
            }
            assert!(!state.step());
        }
        for i in 0 .. N {
            if states[i].input.is_empty() {
                states[i].input.push_back(-1);
                idle[i] += 1;
                assert!(states[i].output.is_empty());
            }
        }
        if idle.iter().all(|&x| x > 1) {
            assert!(states.iter().all(|state| state.output.is_empty()));
            assert_eq!(states[0].input.pop_front(), Some(-1));
            assert!(states[0].input.is_empty());
            let (x, y) = nat.unwrap();
            states[0].input.push_back(x);
            states[0].input.push_back(y);
            idle[0] = -1;
            if last_y == Some(y) {
                println!("{}", y);
                return;
            }
            last_y = Some(y);
        }
        for i in 0 .. N {
            assert!(states[i].output.len() <= 3);
            if states[i].output.len() == 3 {
                let j = states[i].output.pop_front().unwrap() as usize;
                let x = states[i].output.pop_front().unwrap();
                let y = states[i].output.pop_front().unwrap();
                if j == 255 {
                    nat = Some((x, y));
                } else {
                    if states[j].input.front() == Some(&-1) {
                        assert_eq!(states[j].input.pop_front(), Some(-1));
                        assert!(states[j].input.is_empty());
                    }
                    states[j].input.push_back(x);
                    states[j].input.push_back(y);
                    idle[j] = -1;
                }
            }
        }
    }
}
