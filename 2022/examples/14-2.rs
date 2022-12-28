use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

enum Cell {
    Rock,
    Sand,
}

fn drop(map: &mut HashMap<Coord, Cell>, limit: i64, mut coord: Coord) {
    'drop: loop {
        assert!(!map.contains_key(&coord));
        for x in [0, -1, 1] {
            let next = Coord { x: coord.x + x, y: coord.y + 1 };
            if next.y < limit && !map.contains_key(&next) {
                coord = next;
                continue 'drop;
            }
        }
        map.insert(coord, Cell::Sand);
        break;
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let mut prev = None;
        for word in line.split(" -> ") {
            let coord = word.split(',').collect::<Vec<_>>();
            assert_eq!(coord.len(), 2);
            let mut coord = Coord { x: coord[0].parse()?, y: coord[1].parse()? };
            let mut prev = match prev.replace(coord) {
                None => continue,
                Some(x) => x,
            };
            if coord < prev {
                std::mem::swap(&mut coord, &mut prev);
            }
            let iter: Box<dyn Iterator<Item = Coord>> = if prev.x == coord.x {
                Box::new((prev.y ..= coord.y).map(|y| Coord { y, ..coord }))
            } else {
                assert_eq!(prev.y, coord.y);
                Box::new((prev.x ..= coord.x).map(|x| Coord { x, ..coord }))
            };
            for coord in iter {
                map.insert(coord, Cell::Rock);
            }
        }
    }
    let limit = map.keys().map(|c| c.y).max().unwrap() + 2;
    let start = Coord { x: 500, y: 0 };
    while !map.contains_key(&start) {
        drop(&mut map, limit, start);
    }
    writeln!(output, "{}", map.values().filter(|x| matches!(x, Cell::Sand)).count())?;
    Ok(())
}

adventofcode::main!(solve("examples/14.txt") == "25055\n");
