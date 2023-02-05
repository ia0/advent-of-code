use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

#[derive(Debug, Copy, Clone)]
enum Slot {
    Register(usize),
    Value(i64),
}
use Slot::*;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Magic,
    Inc(Slot),
    Dec(Slot),
    Tgl(Slot),
    Jnz(Slot, Slot),
    Cpy(Slot, Slot),
}
use Instruction::*;

impl FromStr for Slot {
    type Err = Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match input {
            "a" => Register(0),
            "b" => Register(1),
            "c" => Register(2),
            "d" => Register(3),
            x => Value(x.parse()?),
        })
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(input) = input.strip_prefix("inc ") {
            return Ok(Inc(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("dec ") {
            return Ok(Dec(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("tgl ") {
            return Ok(Tgl(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("jnz ") {
            let (reg, off) = input.split_once(' ').context("bad jnz")?;
            return Ok(Jnz(reg.parse()?, off.parse()?));
        }
        if let Some(input) = input.strip_prefix("cpy ") {
            let (src, dst) = input.split_once(' ').context("bad cpy")?;
            return Ok(Cpy(src.parse()?, dst.parse()?));
        }
        bail!("bad instr");
    }
}

impl Slot {
    fn value(self, regs: &[i64]) -> i64 {
        match self {
            Register(r) => regs[r],
            Value(x) => x,
        }
    }
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match *self {
            Magic => unreachable!(),
            Inc(x) => Dec(x),
            Dec(x) | Tgl(x) => Inc(x),
            Jnz(x, y) => Cpy(x, y),
            Cpy(x, y) => Jnz(x, y),
        };
    }
}

#[derive(Debug)]
struct State {
    program: HashMap<i64, Instruction>,
    register: [i64; 4],
    counter: i64,
}

impl State {
    fn new(program: HashMap<i64, Instruction>) -> State {
        State { program, register: [12, 0, 0, 0], counter: 0 }
    }

    fn step(&mut self) -> bool {
        let mut jump = None;
        match self.program.get(&self.counter).cloned() {
            Some(Magic) => {
                self.register[0] *= self.register[1];
                self.register[1] -= 1;
                self.register[2] = 2 * self.register[1];
                self.register[3] = 0;
                jump = Some(14);
            }
            Some(Inc(Register(r))) => self.register[r] += 1,
            Some(Dec(Register(r))) => self.register[r] -= 1,
            Some(Tgl(x)) => {
                if let Some(x) = self.program.get_mut(&(self.counter + x.value(&self.register))) {
                    x.toggle();
                }
            }
            Some(Jnz(x, _)) if x.value(&self.register) == 0 => (),
            Some(Jnz(_, o)) => jump = Some(o.value(&self.register)),
            Some(Cpy(x, Register(r))) => self.register[r] = x.value(&self.register),
            Some(_) => (),
            None => return false,
        }
        match jump {
            None => self.counter += 1,
            Some(offset) => self.counter += offset,
        }
        true
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let program = BufReader::new(input)
        .lines()
        .enumerate()
        .map(|(i, x)| Result::Ok((i as i64, x?.parse()?)))
        .collect::<Result<HashMap<i64, Instruction>>>()?;
    let mut state = State::new(program);
    for i in 2 .. 16 {
        state.program.remove(&i);
    }
    state.program.insert(2, Magic);
    while state.step() {}
    writeln!(output, "{}", state.register[0])?;
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "479008890\n");
