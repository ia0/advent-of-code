use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let grid: Vec<Vec<u8>> =
        BufReader::new(input).lines().map(|x| x.unwrap().into_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    assert!(grid.iter().all(|x| x.len() == cols));
    let mut left = grid.clone();
    let mut right = grid.clone();
    for row in 0 .. rows {
        for col in 0 .. cols {
            if col > 0 && left[row][col - 1] > left[row][col] {
                left[row][col] = left[row][col - 1];
            }
            if col > 0 && right[row][cols - col] > right[row][cols - col - 1] {
                right[row][cols - col - 1] = right[row][cols - col];
            }
        }
    }
    let mut top = grid.clone();
    let mut bottom = grid.clone();
    for col in 0 .. cols {
        for row in 0 .. rows {
            if row > 0 && top[row - 1][col] > top[row][col] {
                top[row][col] = top[row - 1][col];
            }
            if row > 0 && bottom[rows - row][col] > bottom[rows - row - 1][col] {
                bottom[rows - row - 1][col] = bottom[rows - row][col];
            }
        }
    }
    let mut count = 0;
    for row in 0 .. rows {
        for col in 0 .. cols {
            let mut visible = false;
            visible |= col == 0 || left[row][col - 1] < grid[row][col];
            visible |= col == cols - 1 || right[row][col + 1] < grid[row][col];
            visible |= row == 0 || top[row - 1][col] < grid[row][col];
            visible |= row == rows - 1 || bottom[row + 1][col] < grid[row][col];
            count += visible as usize;
        }
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "1690\n");
