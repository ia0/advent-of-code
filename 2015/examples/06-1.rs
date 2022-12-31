use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn turn_on(x: &mut bool) {
    *x = true;
}

fn turn_off(x: &mut bool) {
    *x = false;
}

fn toggle(x: &mut bool) {
    *x ^= true;
}

fn parse(xy: &str) -> Result<(usize, usize)> {
    let (x, y) = xy.split_once(',').unwrap();
    Ok((x.parse()?, y.parse()?))
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut grid = vec![vec![false; 1000]; 1000];
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (line, action): (_, fn(&mut bool)) = match line.strip_prefix("turn ") {
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
    let total = grid.iter().map(|x| x.iter().filter(|&&x| x).count()).sum::<usize>();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "377891\n");
