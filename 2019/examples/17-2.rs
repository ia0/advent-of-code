extern crate adventofcode;

use std::collections::VecDeque;
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

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

fn is_empty(map: &Vec<Vec<Tile>>, coord: Coord) -> bool {
    if coord.x < 0 || coord.y < 0 {
        return true;
    }
    let x = coord.x as usize;
    let y = coord.y as usize;
    if y >= map.len() || x >= map[0].len() {
        return true;
    }
    map[y][x] == Tile::Empty
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Front { count: usize },
}

fn main() {
    let input = File::open("examples/17.txt").unwrap();
    let line = BufReader::new(input).lines().next().unwrap().unwrap();
    let mut program: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    program[0] = 2;
    let mut state = State::new(program);
    let mut map: Vec<Vec<_>> = Vec::new();
    let mut line = Vec::new();
    let mut robot = None;
    let mut cur = Coord::default();
    'parse: while !state.step() {
        while let Some(x) = state.output.pop_front() {
            match u8::try_from(x).unwrap() {
                b'\n' => {
                    if line.is_empty() {
                        break 'parse;
                    }
                    if let Some(prev) = map.last() {
                        assert_eq!(prev.len(), line.len());
                    }
                    map.push(line);
                    cur.y += 1;
                    cur.x = 0;
                    line = Vec::new()
                }
                b'.' => {
                    line.push(Tile::Empty);
                    cur.x += 1;
                }
                b'#' => {
                    line.push(Tile::Wall);
                    cur.x += 1;
                }
                b'^' => {
                    line.push(Tile::Wall);
                    assert!(robot.is_none());
                    robot = Some(cur);
                    cur.x += 1;
                }
                _ => unreachable!(),
            }
        }
    }
    let mut robot = robot.unwrap();
    let mut dir = Coord { x: 0, y: -1 };
    let mut moves = Vec::new();
    loop {
        let left = Coord { x: dir.y, y: -dir.x };
        let right = Coord { x: -dir.y, y: dir.x };
        let front = robot + dir;
        if !is_empty(&map, front) {
            match moves.last_mut() {
                Some(Move::Front { count }) => *count += 1,
                _ => moves.push(Move::Front { count: 1 }),
            };
            robot = front;
        } else if !is_empty(&map, robot + left) {
            assert!(is_empty(&map, robot + right));
            moves.push(Move::Left);
            dir = left;
        } else if !is_empty(&map, robot + right) {
            moves.push(Move::Right);
            dir = right;
        } else {
            break;
        }
    }
    eprintln!("{:?}", moves);
    // A,C,A,C,B,B,C,A,C,B
    // A = L,12,L,10,R,8,L,12
    // B = L,10,R,12,R,8
    // C = R,8,R,10,R,12
    for &x in b"A,C,A,C,B,B,C,A,C,B\nL,12,L,10,R,8,L,12\nL,10,R,12,R,8\nR,8,R,10,R,12\nn\n".iter() {
        state.input.push_back(x as i64);
    }
    while !state.step() {
        while let Some(x) = state.output.pop_front() {
            match u8::try_from(x) {
                Ok(x) => eprint!("{}", x as char),
                Err(_) => println!("{}", x),
            }
        }
    }
}
