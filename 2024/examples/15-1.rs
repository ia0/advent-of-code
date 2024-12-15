use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Box,
    Wall,
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
            let pos = Coord { x, y };
            match byte {
                b'#' => assert!(map.insert(pos, Cell::Wall).is_none()),
                b'O' => assert!(map.insert(pos, Cell::Box).is_none()),
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
            let mut pos = robot + dir;
            loop {
                match map.get(&pos).cloned() {
                    Some(Cell::Wall) => break,
                    Some(Cell::Box) => pos += dir,
                    None => {
                        if pos != robot + dir {
                            assert_eq!(map.remove(&(robot + dir)), Some(Cell::Box));
                            assert!(map.insert(pos, Cell::Box).is_none());
                        }
                        robot += dir;
                        break;
                    }
                }
            }
        }
    }
    let mut total = 0;
    for (pos, cell) in map {
        if cell == Cell::Box {
            total += pos.x + 100 * pos.y;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "1559280\n");
