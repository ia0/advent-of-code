use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

struct Problem<'a> {
    row: &'a [u8],
    cnt: &'a [i32],
}

impl<'a> Problem<'a> {
    fn count(&self, i: usize, j: usize) -> i64 {
        match (i == self.row.len(), j == self.cnt.len()) {
            (true, true) => return 1,
            (true, false) => return 0,
            (false, true) => return self.row[i ..].iter().all(|x| matches!(x, b'.' | b'?')) as i64,
            (false, false) => (),
        }
        match self.row[i] {
            b'.' => self.count(i + 1, j),
            b'#' => self.eat(i + 1, j + 1, self.cnt[j] - 1),
            b'?' => self.count(i + 1, j) + self.eat(i + 1, j + 1, self.cnt[j] - 1),
            _ => unreachable!(),
        }
    }

    fn eat(&self, i: usize, j: usize, k: i32) -> i64 {
        if i == self.row.len() {
            return (k == 0 && j == self.cnt.len()) as i64;
        }
        match (self.row[i], k) {
            (b'.' | b'?', 0) => self.count(i + 1, j),
            (b'#', 0) | (b'.', _) => 0,
            (b'#' | b'?', _) => self.eat(i + 1, j, k - 1),
            _ => unreachable!(),
        }
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (row, cnt) = line.split_once(' ').context("no space")?;
        let row = row.as_bytes();
        let cnt = cnt.split(',').map(|x| Ok(x.parse()?)).collect::<Result<Vec<i32>>>()?;
        ensure!(row.iter().all(|x| matches!(x, b'.' | b'#' | b'?')));
        ensure!(cnt.iter().all(|&x| 0 < x));
        total += Problem { row, cnt: &cnt }.count(0, 0);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "7110\n");
