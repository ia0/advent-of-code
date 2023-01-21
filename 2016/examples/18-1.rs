use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn get(grid: &HashMap<Coord, u8>, coord: Coord) -> u8 {
    grid.get(&coord).cloned().unwrap_or(b'.')
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut grid = (lines.next().unwrap()?.bytes().enumerate())
        .map(|(i, x)| (Coord { x: i as i64, y: 0 }, x))
        .collect::<HashMap<Coord, u8>>();
    let nx = grid.len() as i64;
    assert!(lines.next().is_none());
    for y in 1 .. 40 {
        for x in 0 .. nx {
            let left = Coord { x: x - 1, y: y - 1 };
            let right = Coord { x: x + 1, y: y - 1 };
            let tile = if get(&grid, left) == get(&grid, right) { b'.' } else { b'^' };
            grid.insert(Coord { x, y }, tile);
        }
    }
    writeln!(output, "{}", grid.values().filter(|&&x| x == b'.').count())?;
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "2016\n");
