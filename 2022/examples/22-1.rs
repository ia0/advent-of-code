use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Move(i64),
    Turn(char),
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    let mut lines = BufReader::new(input).lines();
    let mut pos = None;
    for (y, line) in (1 ..).zip(&mut lines) {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (x, tile) in (1 ..).zip(line.bytes()) {
            let coord = Coord { x, y };
            if pos.is_none() && tile != b' ' {
                pos = Some(coord);
            }
            let tile = match tile {
                b'.' => Tile::Empty,
                b'#' => Tile::Wall,
                b' ' => continue,
                _ => unreachable!(),
            };
            assert!(map.insert(coord, tile).is_none());
        }
    }
    let line = lines.next().unwrap()?;
    assert!(lines.next().is_none());
    let mut instructions = Vec::new();
    let mut dist = 0;
    for c in line.bytes() {
        match c {
            b'0' ..= b'9' => dist = 10 * dist + (c - b'0') as i64,
            b'R' | b'L' => {
                assert!(dist > 0);
                instructions.push(Instruction::Move(dist));
                dist = 0;
                instructions.push(Instruction::Turn(c as char));
            }
            _ => unreachable!(),
        }
    }
    if dist > 0 {
        instructions.push(Instruction::Move(dist));
    }
    let mut pos = pos.unwrap();
    const DIRS: [Coord; 4] =
        [Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }, Coord { x: -1, y: 0 }, Coord { x: 0, y: -1 }];
    let mut dir = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Move(d) => {
                for _ in 0 .. d {
                    let mut next = pos + DIRS[dir];
                    if !map.contains_key(&next) {
                        let odir = (dir + 2) % 4;
                        next = pos;
                        while map.contains_key(&next) {
                            next += DIRS[odir];
                        }
                        next += DIRS[dir];
                    }
                    match map[&next] {
                        Tile::Empty => pos = next,
                        Tile::Wall => break,
                    }
                }
            }
            Instruction::Turn('R') => dir = (dir + 1) % 4,
            Instruction::Turn('L') => dir = (dir + 3) % 4,
            Instruction::Turn(_) => unreachable!(),
        }
    }
    writeln!(output, "{}", 1000 * pos.y + 4 * pos.x + dir as i64)?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "197160\n");
