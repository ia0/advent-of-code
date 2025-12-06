use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Intervals;
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut problem = Vec::new();
    let mut intervals = Intervals::default();
    for line in BufReader::new(input).lines() {
        let line = line?;
        for (byte, i) in line.bytes().zip(0 ..) {
            if byte != b' ' {
                intervals.insert(i .. i + 1);
            }
        }
        problem.push(line);
    }
    let operators = problem.pop().unwrap();
    assert!(problem.iter().all(|x| x.len() == operators.len()));
    let mut total = 0;
    for range in intervals.iter() {
        let range = range.start as usize .. range.end as usize;
        let values = range.clone().map(|i| {
            let mut number = 0;
            for row in &problem {
                if row.as_bytes()[i] != b' ' {
                    number = 10 * number + (row.as_bytes()[i] - b'0') as i64;
                }
            }
            number
        });
        total += match operators[range].trim() {
            "+" => values.sum::<i64>(),
            "*" => values.product(),
            _ => panic!(),
        };
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "10875057285868\n");
