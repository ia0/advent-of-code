use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{bail, Context, Result};

type Monkey = [u8; 4];

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Number {
    Ready(i64),
    Waiting { left: Monkey, right: Monkey, operation: Operation },
}

impl Operation {
    fn apply(self, left: i64, right: i64) -> i64 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

fn parse(line: std::io::Result<String>) -> Result<(Monkey, Number)> {
    let line = line?;
    let (monkey, number) = line.split_once(": ").context("split")?;
    let monkey = monkey.as_bytes().try_into()?;
    let number = match number.split_whitespace().collect::<Vec<_>>().as_slice() {
        [ready] => Number::Ready(ready.parse()?),
        [left, operation, right] => {
            let left = left.as_bytes().try_into()?;
            let right = right.as_bytes().try_into()?;
            let operation = match *operation {
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "*" => Operation::Mul,
                "/" => Operation::Div,
                x => bail!("invalid operation {x}"),
            };
            Number::Waiting { left, right, operation }
        }
        x => bail!("invalid number {x:?}"),
    };
    Ok((monkey, number))
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut monkeys: HashMap<Monkey, Number> =
        BufReader::new(input).lines().map(parse).collect::<Result<_>>()?;
    let mut todo = vec![(*b"root", None)];
    let mut stack = Vec::new();
    while let Some((monkey, operation)) = todo.pop() {
        match operation {
            None => match &monkeys[&monkey] {
                Number::Ready(x) => stack.push(*x),
                Number::Waiting { left, right, operation } => {
                    todo.push((monkey, Some(*operation)));
                    todo.push((*left, None));
                    todo.push((*right, None));
                }
            },
            Some(operation) => {
                let left = stack.pop().context("pop")?;
                let right = stack.pop().context("pop")?;
                let ready = operation.apply(left, right);
                stack.push(ready);
                monkeys.insert(monkey, Number::Ready(ready));
            }
        }
    }
    assert_eq!(stack.len(), 1);
    writeln!(output, "{}", stack[0])?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "299983725663456\n");
