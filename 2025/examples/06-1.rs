use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut values = Vec::<Vec<String>>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        values.push(line.split_whitespace().map(|x| x.to_string()).collect());
    }
    let operators = values.pop().unwrap();
    assert!(values.iter().all(|x| x.len() == operators.len()));
    let mut total = 0;
    for (i, operator) in operators.iter().enumerate() {
        let values = values.iter().map(|x| x[i].parse::<i64>().unwrap());
        total += match operator.as_str() {
            "+" => values.sum::<i64>(),
            "*" => values.product(),
            _ => panic!(),
        };
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/06.txt") == "4580995422905\n");
