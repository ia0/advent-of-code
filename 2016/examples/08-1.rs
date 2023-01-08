use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;
use regex::Regex;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut grid = HashSet::new();
    let rect = Regex::new("^rect (.*)x(.*)$")?;
    let rotrow = Regex::new("^rotate row y=(.*) by (.*)$")?;
    let rotcol = Regex::new("^rotate column x=(.*) by (.*)$")?;
    for line in BufReader::new(input).lines() {
        let line = line?;
        if let Some(captures) = rect.captures(&line) {
            for x in 0 .. captures[1].parse()? {
                for y in 0 .. captures[2].parse()? {
                    grid.insert(Coord { x, y });
                }
            }
        } else if let Some(captures) = rotrow.captures(&line) {
            let y = captures[1].parse()?;
            let mut row = (0 .. 50).map(|x| grid.remove(&Coord { x, y })).collect::<Vec<_>>();
            row.rotate_right(captures[2].parse()?);
            for x in 0 .. 50 {
                if row[x as usize] {
                    grid.insert(Coord { x, y });
                }
            }
        } else if let Some(captures) = rotcol.captures(&line) {
            let x = captures[1].parse()?;
            let mut col = (0 .. 6).map(|y| grid.remove(&Coord { x, y })).collect::<Vec<_>>();
            col.rotate_right(captures[2].parse()?);
            for y in 0 .. 6 {
                if col[y as usize] {
                    grid.insert(Coord { x, y });
                }
            }
        } else {
            unreachable!();
        }
    }
    writeln!(output, "{}", grid.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "123\n");
