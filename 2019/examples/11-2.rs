use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

fn print(panels: &HashMap<Coord, i64>) {
    #[derive(Debug)]
    struct Frame {
        min: Coord,
        max: Coord,
    }
    let frame = panels.keys().fold(
        Frame { min: Coord { x: 0, y: 0 }, max: Coord { x: 0, y: 0 } },
        |mut frame, coord| {
            frame.min.x = std::cmp::min(frame.min.x, coord.x);
            frame.min.y = std::cmp::min(frame.min.y, coord.y);
            frame.max.x = std::cmp::max(frame.max.x, coord.x);
            frame.max.y = std::cmp::max(frame.max.y, coord.y);
            frame
        },
    );
    for y in (frame.min.y ..= frame.max.y).rev() {
        for x in frame.min.x ..= frame.max.x {
            match panels.get(&Coord { x, y }).cloned().unwrap_or(0) {
                0 => print!("."),
                1 => print!("#"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

fn main() {
    let input = File::open("examples/11.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let program = line.split(',').map(|x| x.parse().unwrap()).collect();
    let mut state = State::new(program);
    let mut panels = HashMap::new();
    panels.insert(Coord { x: 0, y: 0 }, 1);
    let mut pos = Coord { x: 0, y: 0 };
    let mut dir = Coord { x: 0, y: 1 };
    loop {
        let color = panels.get(&pos).cloned();
        state.input.push_back(color.unwrap_or(0));
        while state.output.len() != 2 {
            if state.step() {
                assert_eq!(state.output.len(), 0);
                print(&panels);
                return;
            }
        }
        let color = state.output.pop_front().unwrap();
        assert!(color == 0 || color == 1);
        panels.insert(pos, color);
        let turn = state.output.pop_front().unwrap();
        dir = match turn {
            0 => Coord { x: -dir.y, y: dir.x },
            1 => Coord { x: dir.y, y: -dir.x },
            _ => unreachable!(),
        };
        pos.x += dir.x;
        pos.y += dir.y;
    }
}
