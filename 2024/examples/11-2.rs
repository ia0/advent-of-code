use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::parse_ints;
use anyhow::Result;

#[derive(Clone, Copy)]
enum Action {
    Update(i64),
    Split(i64, i64),
}

fn blink(x: i64) -> Action {
    if x == 0 {
        Action::Update(1)
    } else if x.ilog10() % 2 == 1 {
        let m = 10i64.pow(x.ilog10() / 2 + 1);
        Action::Split(x / m, x % m)
    } else {
        Action::Update(x * 2024)
    }
}

fn count(cache: &mut HashMap<(i64, u8), u64>, x: i64, n: u8) -> u64 {
    if n == 0 {
        return 1;
    }
    if let Some(&c) = cache.get(&(x, n)) {
        return c;
    }
    let c = match blink(x) {
        Action::Update(x) => count(cache, x, n - 1),
        Action::Split(x, y) => count(cache, x, n - 1) + count(cache, y, n - 1),
    };
    cache.insert((x, n), c);
    c
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let xs = parse_ints(&lines.next().unwrap()?)?;
    assert!(lines.next().is_none());
    let mut cache = HashMap::new();
    let mut total = 0;
    for x in xs {
        total += count(&mut cache, x, 75);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "244782991106220\n");
