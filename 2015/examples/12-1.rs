use std::io::{Read, Write};

use anyhow::Result;
use serde_json::Value;

fn sum(json: &Value) -> i64 {
    match json {
        Value::Null | Value::Bool(_) => unreachable!(),
        Value::Number(x) => x.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(x) => x.iter().map(sum).sum(),
        Value::Object(x) => x.values().map(sum).sum(),
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let json: Value = serde_json::from_reader(input)?;
    writeln!(output, "{}", sum(&json))?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "156366\n");
