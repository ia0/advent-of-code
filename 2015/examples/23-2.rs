use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

struct Register(usize);

type Offset = i64;

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "a" => Ok(Register(0)),
            "b" => Ok(Register(1)),
            _ => bail!("bad register {input}"),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(input) = input.strip_prefix("hlf ") {
            return Ok(Instruction::Half(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("tpl ") {
            return Ok(Instruction::Triple(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("inc ") {
            return Ok(Instruction::Increment(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("jmp ") {
            return Ok(Instruction::Jump(input.parse()?));
        }
        if let Some(input) = input.strip_prefix("jie ") {
            let (register, offset) =
                input.split_once(", ").with_context(|| format!("bad instruction {input}"))?;
            return Ok(Instruction::JumpIfEven(register.parse()?, offset.parse()?));
        }
        if let Some(input) = input.strip_prefix("jio ") {
            let (register, offset) =
                input.split_once(", ").with_context(|| format!("bad instruction {input}"))?;
            return Ok(Instruction::JumpIfOne(register.parse()?, offset.parse()?));
        }
        bail!("bad instruction {input}");
    }
}

struct State {
    program: HashMap<i64, Instruction>,
    register: [usize; 2],
    counter: i64,
}

impl State {
    fn new(program: HashMap<i64, Instruction>) -> State {
        State { program, register: [1, 0], counter: 0 }
    }

    fn step(&mut self) -> bool {
        let mut jump = None;
        match self.program.get(&self.counter) {
            Some(Instruction::Half(r)) => self.register[r.0] /= 2,
            Some(Instruction::Triple(r)) => self.register[r.0] *= 3,
            Some(Instruction::Increment(r)) => self.register[r.0] += 1,
            Some(Instruction::Jump(o)) => jump = Some(o),
            Some(Instruction::JumpIfEven(r, o)) if self.register[r.0] % 2 == 0 => jump = Some(o),
            Some(Instruction::JumpIfEven(_, _)) => (),
            Some(Instruction::JumpIfOne(r, o)) if self.register[r.0] == 1 => jump = Some(o),
            Some(Instruction::JumpIfOne(_, _)) => (),
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
    writeln!(output, "{}", state.register[1])?;
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "247\n");
