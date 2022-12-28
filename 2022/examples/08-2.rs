use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn count(mut iter: impl Iterator<Item = u8>) -> usize {
    let limit = iter.next().unwrap();
    let mut count = 0;
    for val in iter {
        count += 1;
        if limit <= val {
            break;
        }
    }
    count
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let grid: Vec<Vec<u8>> =
        BufReader::new(input).lines().map(|x| x.unwrap().into_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    assert!(grid.iter().all(|x| x.len() == cols));
    let mut best = 0;
    for row in 0 .. rows {
        for col in 0 .. cols {
            let left = count(grid[row][..= col].iter().rev().cloned());
            let right = count(grid[row][col ..].iter().cloned());
            let top = count(grid[..= row].iter().rev().map(|row| row[col]));
            let down = count(grid[row ..].iter().map(|row| row[col]));
            best = std::cmp::max(best, left * right * top * down);
        }
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "535680\n");
