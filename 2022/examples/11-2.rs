use std::io::{BufRead, BufReader, Lines, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Error, Result};

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Argument(Option<usize>);

#[derive(Debug)]
struct Inspect {
    operation: Operation,
    left: Argument,
    right: Argument,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    inspect: Inspect,
    divisor: usize,
    destination: [usize; 2],
    count: usize,
}

#[derive(Debug)]
struct Business {
    monkeys: Vec<Monkey>,
    modulo: usize,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => bail!("unknown operation: {input:?}"),
        })
    }
}

impl FromStr for Argument {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "old" {
            return Ok(Argument(None));
        }
        Ok(Argument(Some(input.parse()?)))
    }
}

impl FromStr for Inspect {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let words = input.split_whitespace().collect::<Vec<_>>();
        assert_eq!(words.len(), 3);
        let left = words[0].parse()?;
        let operation = words[1].parse()?;
        let right = words[2].parse()?;
        Ok(Inspect { operation, left, right })
    }
}

impl Inspect {
    fn update(&self, old: usize) -> usize {
        let left = self.left.0.unwrap_or(old);
        let right = self.right.0.unwrap_or(old);
        match self.operation {
            Operation::Add => left + right,
            Operation::Mul => left * right,
        }
    }
}

impl Business {
    fn new() -> Self {
        Business { monkeys: Vec::new(), modulo: 1 }
    }

    fn parse(&mut self, lines: &mut Lines<BufReader<impl Read>>) -> Result<()> {
        let line = lines.next().unwrap()?;
        let line = line.strip_prefix("Monkey ").unwrap();
        let line = line.strip_suffix(':').unwrap();
        assert_eq!(line.parse::<usize>()?, self.monkeys.len());
        let line = lines.next().unwrap()?;
        let line = line.strip_prefix("  Starting items: ").unwrap();
        let items = line.split(", ").map(|x| x.parse()).collect::<Result<Vec<usize>, _>>()?;
        let line = lines.next().unwrap()?;
        let line = line.strip_prefix("  Operation: new = ").unwrap();
        let inspect = line.parse()?;
        let line = lines.next().unwrap()?;
        let line = line.strip_prefix("  Test: divisible by ").unwrap();
        let divisor = line.parse()?;
        let mut destination = [0; 2];
        let line = lines.next().unwrap()?;
        let line = line.strip_prefix("    If true: throw to monkey ").unwrap();
        destination[1] = line.parse()?;
        let line = lines.next().unwrap()?;
        let line = line.strip_prefix("    If false: throw to monkey ").unwrap();
        destination[0] = line.parse()?;
        self.modulo *= divisor;
        self.monkeys.push(Monkey { items, inspect, divisor, destination, count: 0 });
        Ok(())
    }

    fn round(&mut self) {
        for i in 0 .. self.monkeys.len() {
            self.turn(i);
        }
    }

    fn turn(&mut self, monkey: usize) {
        for item in 0 .. self.monkeys[monkey].items.len() {
            self.monkeys[monkey].count += 1;
            let mut worry = self.monkeys[monkey].items[item];
            worry = self.monkeys[monkey].inspect.update(worry) % self.modulo;
            let test = worry.is_multiple_of(self.monkeys[monkey].divisor);
            let destination = self.monkeys[monkey].destination[test as usize];
            self.monkeys[destination].items.push(worry);
        }
        self.monkeys[monkey].items.clear();
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut business = Business::new();
    let mut lines = BufReader::new(input).lines();
    loop {
        business.parse(&mut lines)?;
        match lines.next() {
            Some(x) => assert!(x?.is_empty()),
            None => break,
        }
    }
    for _ in 0 .. 10000 {
        business.round();
    }
    writeln!(
        output,
        "{}",
        adventofcode::topn(2, business.monkeys.iter().map(|x| x.count)).product::<usize>()
    )?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "14636993466\n");
