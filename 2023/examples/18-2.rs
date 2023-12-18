#![feature(slice_group_by)]

use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{order_pair, Coord, EAST, NORTH, SOUTH, WEST};
use anyhow::{bail, ensure, Context, Result};

fn weight(ys_a: &[i64], ys_b: &[i64]) -> i64 {
    assert!(ys_a.len() % 2 == 0);
    assert!(ys_b.len() % 2 == 0);
    let mut i_a = 0;
    let mut i_b = 0;
    let mut prev_y = 0;
    let mut total = 0;
    loop {
        let is_a = match (ys_a.get(i_a), ys_b.get(i_b)) {
            (None, None) => break,
            (None, Some(_)) => false,
            (Some(_), None) => true,
            (Some(y_a), Some(y_b)) if y_a == y_b => {
                assert!(i_a & 1 == i_b & 1);
                match i_a & 1 == 0 {
                    true => total += 1,
                    false => total += y_a - prev_y,
                }
                i_a += 1;
                i_b += 1;
                prev_y = *y_a;
                continue;
            }
            (Some(y_a), Some(y_b)) => y_a < y_b,
        };
        let ys = if is_a { ys_a } else { ys_b };
        let (i, j) = if is_a { (&mut i_a, i_b) } else { (&mut i_b, i_a) };
        let y = ys[*i];
        match *i & 1 == 0 && j & 1 == 0 {
            true => total += 1,
            false => total += y - prev_y,
        }
        *i += 1;
        prev_y = y;
    }
    total
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut vert = Vec::new(); // (x, y1, y2)
    let mut pos = Coord::default();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let words = line.split_whitespace().collect::<Vec<_>>();
        ensure!(words.len() == 3);
        let hexa = words[2].as_bytes();
        let hexa = hexa.strip_prefix(b"(#").context("no prefix")?;
        let hexa = hexa.strip_suffix(b")").context("no suffix")?;
        let dir = match hexa[5] {
            b'0' => EAST,
            b'1' => SOUTH,
            b'2' => WEST,
            b'3' => NORTH,
            x => bail!("unexpected direction {x:?}"),
        };
        let len = i64::from_str_radix(std::str::from_utf8(&hexa[.. 5])?, 16)?;
        let next = pos + dir * len;
        if pos.x == next.x {
            let (y1, y2) = order_pair(pos.y, next.y);
            vert.push((pos.x, y1, y2));
        } else {
            assert!(pos.y == next.y);
        }
        pos = next;
    }
    vert.sort();
    let mut prev_x = i64::MIN + 1;
    let mut prev_ys = Vec::new();
    let mut total = 0;
    for lines in vert.group_by(|a, b| a.0 == b.0) {
        let next_x = lines[0].0;
        let mut next_ys = prev_ys.clone();
        for (_, y1, y2) in lines {
            for y in [y1, y2] {
                match next_ys.binary_search(y) {
                    Ok(i) => drop(next_ys.remove(i)),
                    Err(i) => next_ys.insert(i, *y),
                }
            }
        }
        total += (next_x - prev_x - 1) * weight(&prev_ys, &prev_ys);
        total += weight(&prev_ys, &next_ys);
        prev_x = next_x;
        prev_ys = next_ys;
    }
    assert!(prev_ys.is_empty());
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/18.txt") == "177243763226648\n");
