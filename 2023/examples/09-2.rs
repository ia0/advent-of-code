use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn parse(input: std::io::Result<String>) -> Result<Vec<i64>> {
    input?.split_whitespace().map(|x| Ok(x.parse()?)).collect()
}

fn next(mut record: Vec<i64>) -> i64 {
    record.reverse();
    let mut last = vec![record[0]];
    for y in &record[1 ..] {
        let mut y = *y;
        for x in &mut last {
            let d = y - *x;
            *x = y;
            y = d;
        }
        if y != 0 {
            last.push(y);
        }
    }
    last.iter().rfold(0, |x, y| x + y)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let records = BufReader::new(input).lines().map(parse).collect::<Result<Vec<_>>>()?;
    let total = records.into_iter().map(next).sum::<i64>();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "1136\n");
