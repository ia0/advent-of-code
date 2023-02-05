use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{shortest_dist_plus, Coord};
use anyhow::Result;
use number_encoding::factoradics;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut locations = HashMap::new();
    let mut walls = HashSet::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            match byte {
                b'#' => assert!(walls.insert(pos)),
                b'.' => (),
                b'0' ..= b'9' => assert!(locations.insert(byte - b'0', pos).is_none()),
                _ => unreachable!(),
            }
        }
    }
    let mut dists = HashMap::new();
    let n = locations.len() as u8;
    for i in 1 .. n {
        for j in 0 .. i {
            let d = shortest_dist_plus(&walls, locations[&i], locations[&j]).unwrap();
            dists.insert((i, j), d);
            dists.insert((j, i), d);
        }
    }
    let mut locations = (1 .. n).collect::<Vec<_>>();
    let mut iter = factoradics::Iter::new(&mut locations);
    let mut best = usize::MAX;
    while let Some(locations) = iter.next() {
        let mut dist = 0;
        let mut cur = 0;
        for &next in locations {
            dist += dists[&(cur, next)];
            cur = next;
        }
        dist += dists[&(cur, 0)];
        best = std::cmp::min(best, dist);
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/24.txt") == "652\n");
