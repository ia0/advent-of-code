use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame};
use anyhow::Result;
use regex::Regex;

struct Data {
    size: usize,
    used: usize,
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut grid = HashMap::new();
    let regex = Regex::new("^/dev/grid/node-x([0-9]+)-y([0-9]+) +(.+)T +(.+)T +(.+)T +(.*)%$")?;
    for line in BufReader::new(input).lines().skip(2) {
        let line = line?;
        let captures = regex.captures(&line).unwrap();
        let x = captures[1].parse()?;
        let y = captures[2].parse()?;
        let pos = Coord { x, y };
        let size = captures[3].parse()?;
        let used = captures[4].parse()?;
        let avail = captures[5].parse()?;
        let ratio: usize = captures[6].parse()?;
        let data = Data { size, used };
        assert!(grid.insert(pos, data).is_none());
        assert_eq!(size - used, avail);
        assert!((ratio * size / 100 ..= (ratio + 1) * size / 100).contains(&used));
    }
    let frame = Frame::new(grid.keys().cloned()).unwrap();
    let (empty, _) = grid.iter().find(|(_, x)| x.used == 0).unwrap();
    let wall = Frame::new(grid.iter().filter(|(_, x)| 500 <= x.size).map(|(&k, _)| k)).unwrap();
    writeln!(output, "{}", empty.y + 6 * frame.max.x + empty.x - 2 * wall.min.x - 3)?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "242\n");
