use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

type Tri = [u8; 3];

fn trace(instrs: &[u8], map: &HashMap<Tri, [Tri; 2]>, start: &Tri) -> (Vec<i64>, i64, i64) {
    let mut rems = Vec::new();
    let mut total = 0;
    let mut cur = *start;
    let mut visited = HashMap::new();
    loop {
        let pos = total % instrs.len();
        if let Some(prev) = visited.insert((cur, pos), total as i64) {
            return (rems, prev, total as i64);
        }
        cur = match instrs[pos] {
            b'L' => map[&cur][0],
            b'R' => map[&cur][1],
            _ => unreachable!(),
        };
        total += 1;
        if cur[2] == b'Z' {
            rems.push(total as i64);
        }
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let instrs = lines.next().context("no instructions")??.into_bytes();
    ensure!(instrs.iter().all(|x| b"LR".contains(x)));
    ensure!(lines.next().context("no empty line")??.is_empty());
    let mut map = HashMap::new();
    for line in lines {
        let line = line?;
        let (src, dst) = line.split_once(" = ").context("no equal")?;
        let dst = dst.strip_prefix('(').context("no opening parenthesis")?;
        let dst = dst.strip_suffix(')').context("no closing parenthesis")?;
        let (left, right) = dst.split_once(", ").context("no comma")?;
        let src = Tri::try_from(src.as_bytes()).context("src no trigram")?;
        let left = Tri::try_from(left.as_bytes()).context("left no trigram")?;
        let right = Tri::try_from(right.as_bytes()).context("right no trigram")?;
        ensure!(map.insert(src, [left, right]).is_none());
    }
    let mut total = 1;
    for (rems, beg, end) in map.keys().filter(|x| x[2] == b'A').map(|x| trace(&instrs, &map, x)) {
        ensure!(rems.len() == 1);
        let rem = rems[0];
        ensure!(rem + beg == end);
        total *= rem / adventofcode::egcd(total, rem).0;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "23977527174353\n");
