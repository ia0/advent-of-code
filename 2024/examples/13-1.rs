use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn parse(x: &str) -> Result<Coord> {
    let (_, x) = x.split_once(": ").unwrap();
    let (x, y) = x.split_once(", ").unwrap();
    Ok(Coord { x: x[2 ..].parse()?, y: y[2 ..].parse()? })
}

fn cost(a: Coord, b: Coord, p: Coord) -> Option<i64> {
    let zb = a.y * b.x - a.x * b.y;
    let zp = a.y * p.x - a.x * p.y;
    assert!(zb != 0); // apparently the lines are never co-linear
    let rb = zp / zb;
    if zp == rb * zb {
        let ra = (p.x - b.x * rb) / a.x;
        if p.x == ra * a.x + rb * b.x {
            if ra <= 100 {
                Some(3 * ra + rb)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut total = 0;
    loop {
        let a = parse(&lines.next().unwrap()?)?;
        let b = parse(&lines.next().unwrap()?)?;
        let p = parse(&lines.next().unwrap()?)?;
        total += cost(a, b, p).unwrap_or(0);
        if lines.next().is_none() {
            break;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/13.txt") == "33481\n");
