use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use internment::Intern;

#[derive(Copy, Clone)]
enum Input {
    Value(u16),
    Wire(Intern<String>),
    Not(Intern<String>),
    LShift(Intern<String>, u8),
    RShift(Intern<String>, u8),
    And(Intern<String>, Intern<String>),
    Or(Intern<String>, Intern<String>),
}

fn eval(circuit: &mut HashMap<Intern<String>, Input>, wire: Intern<String>) -> u16 {
    let value = match circuit.get(&wire) {
        Some(&Input::Value(x)) => return x,
        Some(&Input::Wire(x)) => eval(circuit, x),
        Some(&Input::Not(x)) => !eval(circuit, x),
        Some(&Input::LShift(x, b)) => eval(circuit, x) << b,
        Some(&Input::RShift(x, b)) => eval(circuit, x) >> b,
        Some(&Input::And(x, y)) => eval(circuit, x) & eval(circuit, y),
        Some(&Input::Or(x, y)) => eval(circuit, x) | eval(circuit, y),
        None => wire.parse().unwrap(),
    };
    circuit.insert(wire, Input::Value(value));
    value
}

fn new(x: &str) -> Intern<String> {
    Intern::new(x.to_string())
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut circuit = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (src, dst) = line.split_once(" -> ").unwrap();
        let words = src.split_whitespace().collect::<Vec<_>>();
        let input = match words.as_slice() {
            [x] => match x.parse() {
                Ok(x) => Input::Value(x),
                Err(_) => Input::Wire(new(x)),
            },
            ["NOT", x] => Input::Not(new(x)),
            [x, "LSHIFT", y] => Input::LShift(new(x), y.parse()?),
            [x, "RSHIFT", y] => Input::RShift(new(x), y.parse()?),
            [x, "AND", y] => Input::And(new(x), new(y)),
            [x, "OR", y] => Input::Or(new(x), new(y)),
            _ => unreachable!(),
        };
        assert!(circuit.insert(new(dst), input).is_none());
    }
    let mut second = circuit.clone();
    second.insert(new("b"), Input::Value(eval(&mut circuit, new("a"))));
    writeln!(output, "{}", eval(&mut second, new("a")))?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "14710\n");
