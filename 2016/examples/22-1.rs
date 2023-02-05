use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;
use regex::Regex;

struct Data {
    size: usize,
    used: usize,
}

impl Data {
    fn avail(&self) -> usize {
        self.size - self.used
    }
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
    let mut xs = grid.values().map(|x| x.used).filter(|&x| 0 < x).collect::<Vec<_>>();
    xs.sort();
    let mut ys = grid.values().map(|x| x.avail()).filter(|&x| 0 < x).collect::<Vec<_>>();
    ys.sort();
    let mut count = 0;
    let mut i = 0;
    for x in xs {
        while i < ys.len() && ys[i] < x {
            i += 1;
        }
        count += ys.len() - i;
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "1007\n");
