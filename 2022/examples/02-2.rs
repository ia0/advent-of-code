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
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => return Err(()),
        })
    }
}

impl Shape {
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

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Won,
            _ => return Err(()),
        })
    }
}

impl Outcome {
    fn shape(self, opponent: Shape) -> Shape {
        match opponent as i8 + self as i8 - 1 {
            0 | 3 => Shape::Rock,
            1 => Shape::Paper,
            2 | -1 => Shape::Scissors,
            _ => unreachable!(),
        }
    }

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
        let outcome: Outcome = words[1].parse().unwrap();
        total += outcome.score() + outcome.shape(opponent).score();
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "13433\n");
