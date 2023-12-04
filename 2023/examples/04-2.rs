use std::collections::{BTreeSet, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{Context, Error, Result};

struct Card {
    winning: BTreeSet<i32>,
    numbers: BTreeSet<i32>,
}

fn parse_numbers(input: &str) -> Result<BTreeSet<i32>> {
    input.split_whitespace().map(|x| Ok(x.parse()?)).collect()
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (_, input) = input.split_once(": ").context("no colon")?;
        let (winning, numbers) = input.split_once(" | ").context("no pipe")?;
        let winning = parse_numbers(winning)?;
        let numbers = parse_numbers(numbers)?;
        Ok(Card { winning, numbers })
    }
}

impl Card {
    fn matches(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    let mut copies = VecDeque::<usize>::new();
    for line in BufReader::new(input).lines() {
        let card: Card = line?.parse()?;
        let count = 1 + copies.pop_front().unwrap_or_default();
        for i in 0 .. card.matches() {
            if i < copies.len() {
                copies[i] += count;
            } else {
                copies.push_back(count);
            }
        }
        total += count;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "5539496\n");
