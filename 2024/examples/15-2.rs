use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, EAST, NORTH, SOUTH, WEST};
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    BoxL,
    BoxR,
    Wall,
}

fn update(
    mut map: HashMap<Coord, Cell>, mut pos: Coord, dir: Coord,
) -> Option<HashMap<Coord, Cell>> {
    pos += dir;
    match dir {
        EAST | WEST => match map.get(&pos).cloned() {
            Some(Cell::Wall) => None,
            Some(prev @ (Cell::BoxL | Cell::BoxR)) => {
                assert_eq!(map.remove(&pos), Some(prev));
                map = update(map, pos, dir)?;
                assert!(map.insert(pos + dir, prev).is_none());
                Some(map)
            }
            None => Some(map),
        },
        NORTH | SOUTH => match map.get(&pos).cloned() {
            Some(Cell::Wall) => None,
            Some(prev @ (Cell::BoxL | Cell::BoxR)) => {
                assert_eq!(map.remove(&pos), Some(prev));
                map = update(map, pos, dir)?;
                let other = match prev {
                    Cell::BoxL => EAST,
                    Cell::BoxR => WEST,
                    _ => unreachable!(),
                };
                map = update(map, pos - dir + other, dir)?;
                assert!(map.insert(pos + dir, prev).is_none());
                Some(map)
            }
            None => Some(map),
        },
        _ => unreachable!(),
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    let mut robot = None;
    let mut lines = BufReader::new(input).lines();
    for (line, y) in (&mut lines).zip(0 ..) {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (byte, x) in line.bytes().zip(0 ..) {
            let x = 2 * x;
            let pos = Coord { x, y };
            match byte {
                b'#' => {
                    assert!(map.insert(pos, Cell::Wall).is_none());
                    assert!(map.insert(pos + EAST, Cell::Wall).is_none());
                }
                b'O' => {
                    assert!(map.insert(pos, Cell::BoxL).is_none());
                    assert!(map.insert(pos + EAST, Cell::BoxR).is_none());
                }
                b'.' => (),
                b'@' => assert!(robot.replace(pos).is_none()),
                _ => unreachable!(),
            }
        }
    }
    let mut robot = robot.unwrap();
    for line in lines {
        for byte in line?.bytes() {
            let dir = Coord::parse_dir(byte)?;
            if let Some(new) = update(map.clone(), robot, dir) {
                map = new;
                robot += dir;
            }
        }
    }
    let mut total = 0;
    for (pos, cell) in map {
        if cell == Cell::BoxL {
            total += pos.x + 100 * pos.y;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "1576353\n");
