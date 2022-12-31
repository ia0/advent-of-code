use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn turn_on(x: &mut usize) {
    *x += 1;
}

fn turn_off(x: &mut usize) {
    *x = x.saturating_sub(1);
}

fn toggle(x: &mut usize) {
    *x += 2;
}

fn parse(xy: &str) -> Result<(usize, usize)> {
    let (x, y) = xy.split_once(',').unwrap();
    Ok((x.parse()?, y.parse()?))
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut grid = vec![vec![0; 1000]; 1000];
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (line, action): (_, fn(&mut usize)) = match line.strip_prefix("turn ") {
            Some(line) => match line.strip_prefix("on ") {
                Some(line) => (line, turn_on),
                None => (line.strip_prefix("off ").unwrap(), turn_off),
            },
            None => (line.strip_prefix("toggle ").unwrap(), toggle),
        };
        let (min, max) = line.split_once(" through ").unwrap();
        let min = parse(min)?;
        let max = parse(max)?;
        for grid in grid.iter_mut().take(max.0 + 1).skip(min.0) {
            grid.iter_mut().take(max.1 + 1).skip(min.1).for_each(action);
        }
    }
    let total = grid.iter().map(|x| x.iter().sum::<usize>()).sum::<usize>();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "14110788\n");
