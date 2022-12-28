use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::iter::repeat;

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

type Iter<'a> = &'a mut dyn Iterator<Item = (i64, i64)>;
struct Wrap<'a>(Iter<'a>, usize, Iter<'a>, usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct State {
    pos: Coord,
    dir: usize,
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
    let mut wrap = HashMap::new();
    for Wrap(a, c, b, d) in [
        Wrap(&mut (51 ..= 100).zip(repeat(0)), 1, &mut repeat(0).zip(151 ..= 200), 0),
        Wrap(&mut (101 ..= 150).zip(repeat(0)), 1, &mut (1 ..= 50).zip(repeat(201)), 3),
        Wrap(&mut repeat(151).zip(1 ..= 50), 2, &mut repeat(101).zip((101 ..= 150).rev()), 2),
        Wrap(&mut (101 ..= 150).zip(repeat(51)), 3, &mut repeat(101).zip(51 ..= 100), 2),
        Wrap(&mut (51 ..= 100).zip(repeat(151)), 3, &mut repeat(51).zip(151 ..= 200), 2),
        Wrap(&mut repeat(0).zip(101 ..= 150), 0, &mut repeat(50).zip((1 ..= 50).rev()), 0),
        Wrap(&mut (1 ..= 50).zip(repeat(100)), 1, &mut repeat(50).zip(51 ..= 100), 0),
    ] {
        for ((ax, ay), (bx, by)) in a.zip(b) {
            let a = Coord { x: ax, y: ay };
            let b = Coord { x: bx, y: by };
            assert!(wrap
                .insert(State { pos: a, dir: (c + 2) % 4 }, State { pos: b, dir: d })
                .is_none());
            assert!(wrap
                .insert(State { pos: b, dir: (d + 2) % 4 }, State { pos: a, dir: c })
                .is_none());
        }
    }
    const DIRS: [Coord; 4] =
        [Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }, Coord { x: -1, y: 0 }, Coord { x: 0, y: -1 }];
    let mut cur = State { pos: pos.unwrap(), dir: 0 };
    for instruction in instructions {
        match instruction {
            Instruction::Move(d) => {
                for _ in 0 .. d {
                    let mut next = cur;
                    next.pos += DIRS[next.dir];
                    if !map.contains_key(&next.pos) {
                        next = wrap[&next];
                        next.pos += DIRS[next.dir];
                    }
                    if matches!(map[&next.pos], Tile::Wall) {
                        break;
                    }
                    cur = next;
                }
            }
            Instruction::Turn('R') => cur.dir = (cur.dir + 1) % 4,
            Instruction::Turn('L') => cur.dir = (cur.dir + 3) % 4,
            Instruction::Turn(_) => unreachable!(),
        }
    }
    writeln!(output, "{}", 1000 * cur.pos.y + 4 * cur.pos.x + cur.dir as i64)?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "145065\n");
