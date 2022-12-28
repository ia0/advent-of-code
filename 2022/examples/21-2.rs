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
use Operation::*;

enum Number {
    Ready(Value),
    Waiting { left: Monkey, right: Monkey, operation: Operation },
}
use Number::*;

#[derive(Clone)]
enum Value {
    Human,
    Immediate(i64),
    Thunk { left: Box<Value>, right: Box<Value>, operation: Operation },
}
use Value::*;

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Human => write!(f, "x"),
            Immediate(x) => write!(f, "{x}"),
            Thunk { left, right, operation } => write!(f, "({left} {operation} {right})"),
        }
    }
}

impl Operation {
    fn apply(self, left: Value, right: Value) -> Value {
        match (left, self, right) {
            (Immediate(left), Add, Immediate(right)) => Immediate(left + right),
            (Immediate(left), Sub, Immediate(right)) => Immediate(left - right),
            (Immediate(left), Mul, Immediate(right)) => Immediate(left * right),
            (Immediate(left), Div, Immediate(right)) => Immediate(left / right),
            (left, operation, right) => {
                Thunk { left: Box::new(left), right: Box::new(right), operation }
            }
        }
    }

    fn reverse(self, human: &mut i64, left: Value, right: Value) -> Result<Value> {
        Ok(match (left, self, right) {
            (Immediate(left), Add, right) => {
                *human -= left;
                right
            }
            (Immediate(left), Sub, right) => {
                *human = left - *human;
                right
            }
            (Immediate(left), Mul, right) => {
                *human /= left;
                right
            }
            (Immediate(left), Div, right) => {
                *human = left / *human;
                right
            }
            (left, Add, Immediate(right)) => {
                *human -= right;
                left
            }
            (left, Sub, Immediate(right)) => {
                *human += right;
                left
            }
            (left, Mul, Immediate(right)) => {
                *human /= right;
                left
            }
            (left, Div, Immediate(right)) => {
                *human *= right;
                left
            }
            (left, operation, right) => bail!("{left} {operation} {right}"),
        })
    }
}

fn parse(line: std::io::Result<String>) -> Result<(Monkey, Number)> {
    let line = line?;
    let (monkey, number) = line.split_once(": ").context("split")?;
    let monkey = monkey.as_bytes().try_into()?;
    let number = match number.split_whitespace().collect::<Vec<_>>().as_slice() {
        [ready] => Ready(Immediate(ready.parse()?)),
        [left, operation, right] => {
            let left = left.as_bytes().try_into()?;
            let right = right.as_bytes().try_into()?;
            let operation = match *operation {
                "+" => Add,
                "-" => Sub,
                "*" => Mul,
                "/" => Div,
                x => bail!("invalid operation {x}"),
            };
            Waiting { left, right, operation }
        }
        x => bail!("invalid number {x:?}"),
    };
    Ok((monkey, number))
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut monkeys: HashMap<Monkey, Number> =
        BufReader::new(input).lines().map(parse).collect::<Result<_>>()?;
    let root = match monkeys.remove(b"root") {
        Some(Waiting { left, right, .. }) => (left, right),
        _ => bail!("root"),
    };
    monkeys.insert(*b"humn", Ready(Human));
    let mut todo = vec![(root.0, None), (root.1, None)];
    let mut stack = Vec::new();
    while let Some((monkey, operation)) = todo.pop() {
        match operation {
            None => match &monkeys[&monkey] {
                Ready(x) => stack.push(x.clone()),
                Waiting { left, right, operation } => {
                    todo.push((monkey, Some(*operation)));
                    todo.push((*left, None));
                    todo.push((*right, None));
                }
            },
            Some(operation) => {
                let left = stack.pop().context("pop")?;
                let right = stack.pop().context("pop")?;
                let ready = operation.apply(left, right);
                stack.push(ready.clone());
                monkeys.insert(monkey, Ready(ready));
            }
        }
    }
    assert_eq!(stack.len(), 2);
    let mut other = stack.pop().context("other")?;
    let mut human = match stack.pop().context("human")? {
        Immediate(x) => x,
        _ => bail!("human"),
    };
    loop {
        other = match other {
            Human => break,
            Immediate(_) => bail!("right"),
            Thunk { left, right, operation } => operation.reverse(&mut human, *left, *right)?,
        };
    }
    writeln!(output, "{human}")?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "3093175982595\n");
