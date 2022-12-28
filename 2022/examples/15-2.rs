use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

fn dist(a: Coord, b: Coord) -> usize {
    let Coord { x, y } = a - b;
    (x.abs() + y.abs()) as usize
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new(); // sensor to beacon
    for line in BufReader::new(input).lines() {
        let line = line?;
        let line = line.strip_prefix("Sensor at x=").unwrap();
        let (sx, line) = line.split_once(", y=").unwrap();
        let (sy, line) = line.split_once(": closest beacon is at x=").unwrap();
        let (bx, by) = line.split_once(", y=").unwrap();
        assert!(map
            .insert(
                Coord { x: sx.parse()?, y: sy.parse()? },
                Coord { x: bx.parse()?, y: by.parse()? }
            )
            .is_none());
    }
    let max = 4000000;
    for y in 0 ..= max {
        let mut covered = adventofcode::Intervals::default();
        for (&sensor, &beacon) in &map {
            let d = dist(sensor, beacon);
            let p = (sensor.y - y).unsigned_abs() as usize;
            if let Some(h) = d.checked_sub(p) {
                let h = h as i64;
                covered.insert(sensor.x - h .. sensor.x + h + 1);
            }
        }
        covered.intersect(0 .. max + 1);
        if covered.len() == max as usize {
            for x in 0 ..= max {
                if !covered.contains(x) {
                    writeln!(output, "{}", 4000000 * x + y)?;
                }
            }
        } else {
            assert_eq!(covered.len(), max as usize + 1);
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "13134039205729\n");
