use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame};
use anyhow::{bail, ensure, Context, Result};

fn shift(xs: &HashSet<i64>, x: i64) -> i64 {
    x + xs.iter().filter(|&&y| y < x).count() as i64
}

fn dist(a: Coord, b: Coord) -> i64 {
    let Coord { x, y } = a - b;
    x.abs() + y.abs()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut galaxies = HashSet::new();
    for (y, row) in BufReader::new(input).lines().enumerate() {
        let y = y as i64;
        for (x, cell) in row?.bytes().enumerate() {
            let x = x as i64;
            let coord = Coord { x, y };
            match cell {
                b'.' => (),
                b'#' => ensure!(galaxies.insert(coord)),
                _ => bail!("unexpected cell {cell:02x}"),
            }
        }
    }
    let frame = Frame::new(galaxies.iter().cloned()).context("no galaxies")?;
    let mut xs = (frame.min.x ..= frame.max.x).collect::<HashSet<_>>();
    let mut ys = (frame.min.y ..= frame.max.y).collect::<HashSet<_>>();
    for galaxy in &galaxies {
        xs.remove(&galaxy.x);
        ys.remove(&galaxy.y);
    }
    let galaxies = galaxies
        .into_iter()
        .map(|pos| Coord { x: shift(&xs, pos.x), y: shift(&ys, pos.y) })
        .collect::<Vec<_>>();
    let mut total = 0;
    for i in 1 .. galaxies.len() {
        for j in 0 .. i {
            total += dist(galaxies[i], galaxies[j]);
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "9418609\n");
