use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{Context, Error, Result};

struct Game {
    id: usize,
    cubes: Vec<[usize; 3]>,
}

fn parse_cubes(input: &str) -> Result<[usize; 3]> {
    let mut result = [0; 3];
    for cube in input.split(", ") {
        let (count, color) = cube.split_once(' ').context("bad cube")?;
        let count = count.parse()?;
        let index = match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => panic!("unexpected color {color:?}"),
        };
        assert!(count != 0);
        assert!(result[index] == 0);
        result[index] = count;
    }
    Ok(result)
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (game, cubes) = input.split_once(": ").context("no colon")?;
        let id = game.strip_prefix("Game ").context("no game")?.parse()?;
        let cubes = cubes.split("; ").map(parse_cubes).collect::<Result<_>>()?;
        Ok(Game { id, cubes })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.cubes.iter().all(|x| x[0] <= 12 && x[1] <= 13 && x[2] <= 14)
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let game: Game = line?.parse()?;
        if game.is_valid() {
            total += game.id;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "2176\n");
