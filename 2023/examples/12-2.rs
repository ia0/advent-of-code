use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

type Key = (usize, usize, Option<i32>);

struct Problem<'a> {
    cache: HashMap<Key, i64>,
    row: &'a [u8],
    cnt: &'a [i32],
}

impl<'a> Problem<'a> {
    fn cached(&mut self, k: Key, f: impl FnOnce(&mut Self) -> i64) -> i64 {
        if let Some(x) = self.cache.get(&k) {
            return *x;
        }
        let v = f(self);
        self.cache.insert(k, v);
        v
    }

    fn count(&mut self, i: usize, j: usize) -> i64 {
        self.cached((i, j, None), |x| x.count_(i, j))
    }

    fn eat(&mut self, i: usize, j: usize, k: i32) -> i64 {
        self.cached((i, j, Some(k)), |x| x.eat_(i, j, k))
    }

    fn count_(&mut self, i: usize, j: usize) -> i64 {
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

    fn eat_(&mut self, i: usize, j: usize, k: i32) -> i64 {
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
        let row = [row; 5].join(&b'?');
        let cnt = [&cnt[..]; 5].concat();
        total += Problem { cache: HashMap::new(), row: &row, cnt: &cnt }.count(0, 0);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "1566786613613\n");
