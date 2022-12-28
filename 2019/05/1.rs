use std::convert::TryFrom;
use std::io::BufRead;

struct State<'a> {
    program: Box<[i64]>,
    position: usize,
    input: &'a [i64],
    output: Vec<i64>,
}

#[derive(PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}

struct Parameter {
    mode: Mode,
    value: i64,
}

impl<'a> State<'a> {
    fn new(program: Box<[i64]>, input: &[i64]) -> State {
        State { program, position: 0, input, output: Vec::new() }
    }

    // Returns whether halted.
    fn step(&mut self) -> bool {
        let opcode = self.program[self.position];
        self.position += 1;
        assert!(opcode >= 0);
        let modes = opcode as usize / 100;
        match opcode % 100 {
            1 => self.i_add(modes),
            2 => self.i_mul(modes),
            3 => self.i_read(modes),
            4 => self.i_write(modes),
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
            _ => unreachable!(),
        };
        let value = self.program[self.position];
        self.position += 1;
        Parameter { mode, value }
    }

    fn read(&self, parameter: Parameter) -> i64 {
        match parameter.mode {
            Mode::Position => self.program[usize::try_from(parameter.value).unwrap()],
            Mode::Immediate => parameter.value,
        }
    }

    fn write(&mut self, parameter: Parameter, value: i64) {
        assert!(parameter.mode == Mode::Position);
        self.program[usize::try_from(parameter.value).unwrap()] = value;
    }

    fn i_add(&mut self, mut modes: usize) {
        let lhs = self.param(&mut modes);
        let rhs = self.param(&mut modes);
        let out = self.param(&mut modes);
        self.write(out, self.read(lhs) + self.read(rhs));
    }

    fn i_mul(&mut self, mut modes: usize) {
        let lhs = self.param(&mut modes);
        let rhs = self.param(&mut modes);
        let out = self.param(&mut modes);
        self.write(out, self.read(lhs) * self.read(rhs));
    }

    fn i_read(&mut self, mut modes: usize) {
        let dst = self.param(&mut modes);
        let value = self.input[0];
        self.input = &self.input[1 ..];
        self.write(dst, value);
    }

    fn i_write(&mut self, mut modes: usize) {
        let src = self.param(&mut modes);
        self.output.push(self.read(src));
    }
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let program: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    let mut state = State::new(program.into_boxed_slice(), &[1]);
    while !state.step() {}
    println!("{}", state.output.last().unwrap());
}
