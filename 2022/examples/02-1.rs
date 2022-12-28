use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::Result;

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => return Err(()),
        })
    }
}

impl Shape {
    fn outcome(self, opponent: Shape) -> Outcome {
        match self as i8 - opponent as i8 {
            -1 | 2 => Outcome::Lost,
            0 => Outcome::Draw,
            1 | -2 => Outcome::Won,
            _ => unreachable!(),
        }
    }

    fn score(self) -> usize {
        self as usize + 1
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    fn score(self) -> usize {
        self as usize * 3
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines().map(|x| x.unwrap()) {
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(words.len(), 2);
        let opponent = words[0].parse().unwrap();
        let shape: Shape = words[1].parse().unwrap();
        total += shape.outcome(opponent).score() + shape.score();
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "13484\n");
