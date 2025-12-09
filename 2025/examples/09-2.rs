use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{ADJACENT_PLUS, Coord, Frame, order_pair_mut};
use anyhow::Result;

fn extract(xs: impl Iterator<Item = i64>) -> Vec<i64> {
    let mut r: Vec<_> = xs.collect::<HashSet<_>>().into_iter().collect();
    r.sort();
    r
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut red = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (x, y) = line.split_once(',').unwrap();
        red.push(Coord { x: x.parse()?, y: y.parse()? });
    }
    let xs = extract(red.iter().map(|a| a.x));
    let ys = extract(red.iter().map(|a| a.y));
    let rx = xs.iter().cloned().zip(1 ..).collect::<HashMap<i64, i64>>();
    let ry = ys.iter().cloned().zip(1 ..).collect::<HashMap<i64, i64>>();
    let mut map = vec![vec![2u8; ys.len() + 2]; xs.len() + 2];
    red.push(red[0]);
    for w in red.windows(2) {
        let mut ax = rx[&w[0].x] as usize;
        let mut ay = ry[&w[0].y] as usize;
        let mut bx = rx[&w[1].x] as usize;
        let mut by = ry[&w[1].y] as usize;
        #[allow(clippy::needless_range_loop)]
        if ax == bx {
            order_pair_mut(&mut ay, &mut by);
            for y in ay ..= by {
                map[ax][y] = 1;
            }
        } else {
            assert_eq!(ay, by);
            order_pair_mut(&mut ax, &mut bx);
            for x in ax ..= bx {
                map[x][ay] = 1;
            }
        }
    }
    red.pop();
    let frame = Frame {
        min: Coord { x: 0, y: 0 },
        max: Coord { x: xs.len() as i64 + 1, y: ys.len() as i64 + 1 },
    };
    let mut todo = vec![frame.min];
    let mut visited = HashSet::new();
    while let Some(next) = todo.pop() {
        if map[next.x as usize][next.y as usize] != 2 || !visited.insert(next) {
            continue;
        }
        map[next.x as usize][next.y as usize] = 0;
        for dir in ADJACENT_PLUS {
            let next = next + dir;
            if frame.contains(next) {
                todo.push(next);
            }
        }
    }
    let mut max = 0;
    for (i, &a) in red.iter().enumerate() {
        'next: for &b in red.iter().take(i) {
            let lx = std::cmp::min(a.x, b.x);
            let ly = std::cmp::min(a.y, b.y);
            let hx = std::cmp::max(a.x, b.x);
            let hy = std::cmp::max(a.y, b.y);
            for x in rx[&lx] ..= rx[&hx] {
                for y in ry[&ly] ..= ry[&hy] {
                    if map[x as usize][y as usize] == 0 {
                        continue 'next;
                    }
                }
            }
            let area = ((hx - lx + 1) * (hy - ly + 1)) as usize;
            max = std::cmp::max(max, area);
        }
    }
    writeln!(output, "{max}")?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "1539238860\n");
