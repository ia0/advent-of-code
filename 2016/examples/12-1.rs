use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

type Value = i64;
type Offset = i64;
struct Register(usize);
enum Slot {
    Register(Register),
    Value(Value),
}

enum Instruction {
    Cpy(Slot, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Slot, Offset),
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "a" => Ok(Register(0)),
            "b" => Ok(Register(1)),
            "c" => Ok(Register(2)),
            "d" => Ok(Register(3)),
            _ => bail!("bad register {input}"),
        }
    }
}

impl FromStr for Slot {
    type Err = Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match input.parse::<Register>() {
            Ok(x) => Slot::Register(x),
            Err(_) => Slot::Value(input.parse()?),
        })
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(input) = input.strip_prefix("cpy ") {
            let (src, dst) = input.split_once(' ').context("bad cpy")?;
            return Ok(Instruction::Cpy(src.parse()?, dst.parse()?));
        }
        if let Some(input) = input.strip_prefix("inc ") {
            return Ok(Instruction::Inc(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("dec ") {
            return Ok(Instruction::Dec(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("jnz ") {
            let (reg, off) = input.split_once(' ').context("bad jnz")?;
            return Ok(Instruction::Jnz(reg.parse()?, off.parse()?));
        }
        bail!("bad instr");
    }
}

impl Slot {
    fn value(&self, regs: &[i64]) -> i64 {
        match self {
            Slot::Register(r) => regs[r.0],
            Slot::Value(x) => *x,
        }
    }
}

struct State {
    program: HashMap<i64, Instruction>,
    register: [i64; 4],
    counter: i64,
}

impl State {
    fn new(program: HashMap<i64, Instruction>) -> State {
        State { program, register: [0; 4], counter: 0 }
    }

    fn step(&mut self) -> bool {
        let mut jump = None;
        match self.program.get(&self.counter) {
            Some(Instruction::Cpy(x, r)) => self.register[r.0] = x.value(&self.register),
            Some(Instruction::Inc(r)) => self.register[r.0] += 1,
            Some(Instruction::Dec(r)) => self.register[r.0] -= 1,
            Some(Instruction::Jnz(x, _)) if x.value(&self.register) == 0 => (),
            Some(Instruction::Jnz(_, o)) => jump = Some(o),
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
    while state.step() {}
    writeln!(output, "{}", state.register[0])?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "318083\n");
