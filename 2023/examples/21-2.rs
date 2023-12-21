use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS, ADJACENT_STAR};
use anyhow::{bail, ensure, Result};

fn propagate(frame: Frame, walls: &HashSet<Coord>, start: Coord) -> Vec<usize> {
    let mut total = vec![1];
    let mut done = HashSet::new();
    let mut prev = HashSet::new();
    prev.insert(start);
    loop {
        let mut next = HashSet::new();
        for &pos in &prev {
            for dir in ADJACENT_PLUS {
                let pos = pos + dir;
                if frame.contains(pos) && !done.contains(&pos) && !walls.contains(&pos) {
                    next.insert(pos);
                }
            }
        }
        let n = total.len();
        let diff = next.len();
        let old = if 1 < n { total[n - 2] } else { 0 };
        let new = old + diff;
        if new == old {
            break;
        }
        total.push(new);
        done = prev;
        prev = next;
    }
    total
}

fn count(xs: &[usize], n: i64) -> usize {
    let n = n as usize;
    let k = xs.len();
    if n < k {
        xs[n]
    } else if (k - 1) % 2 == n % 2 {
        xs[k - 1]
    } else {
        xs[k - 2]
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut frame = Frame::default();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        frame.max.y = y;
        for (cell, x) in line?.bytes().zip(0 ..) {
            frame.max.x = x;
            let pos = Coord { x, y };
            match cell {
                b'.' => (),
                b'#' => ensure!(walls.insert(pos)),
                b'S' => ensure!(start.replace(pos).is_none()),
                _ => bail!("invalid cell {cell:?}"),
            }
        }
    }
    let start = start.unwrap();
    ensure!(frame.max.x == frame.max.y);
    ensure!(start.x == start.y);
    let n = frame.max.x + 1;
    let k = start.x;
    ensure!(n == 2 * k + 1);
    for x in 0 .. n {
        ensure!(!walls.contains(&Coord { x, y: 0 }));
        ensure!(!walls.contains(&Coord { x, y: k }));
        ensure!(!walls.contains(&Coord { x, y: n - 1 }));
    }
    for y in 0 .. n {
        ensure!(!walls.contains(&Coord { x: 0, y }));
        ensure!(!walls.contains(&Coord { x: k, y }));
        ensure!(!walls.contains(&Coord { x: n - 1, y }));
    }
    let mut cache = HashMap::new();
    for x in -1 ..= 1 {
        for y in -1 ..= 1 {
            let dir = Coord { x, y };
            let start = Coord {
                x: match x {
                    -1 => frame.max.x,
                    0 => start.x,
                    1 => 0,
                    _ => unreachable!(),
                },
                y: match y {
                    -1 => frame.max.y,
                    0 => start.y,
                    1 => 0,
                    _ => unreachable!(),
                },
            };
            cache.insert(dir, propagate(frame, &walls, start));
        }
    }
    const STEPS: i64 = 26501365;
    let mut total = count(&cache[&Coord::default()], STEPS);
    for dir in ADJACENT_STAR {
        let cache = &cache[&dir];
        if dir.x * dir.y == 0 {
            let steps = STEPS - (k + 1);
            let mut full = (steps / (2 * n)) as usize;
            let mut rest = steps % (2 * n);
            while rest < cache.len() as i64 {
                full -= 1;
                rest += 2 * n;
            }
            total += full * (count(cache, steps) + count(cache, steps - n));
            while 0 <= rest {
                total += count(cache, rest);
                rest -= n;
            }
        } else {
            let steps = STEPS - 2 * (k + 1);
            let mut full = (steps / (2 * n)) as usize;
            let mut rest = steps % (2 * n);
            while rest < cache.len() as i64 {
                full -= 1;
                rest += 2 * n;
            }
            total += full * (full + 1) * (count(cache, steps) + count(cache, steps - n));
            full += 1;
            total += full * count(cache, rest);
            rest -= n;
            full *= 2;
            while 0 <= rest {
                total += full * count(cache, rest);
                rest -= n;
                full += 1;
            }
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "602259568764234\n");
