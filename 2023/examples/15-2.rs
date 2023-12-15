use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{bail, ensure, Context, Result};

fn hash(xs: &[u8]) -> u8 {
    let mut r = 0u8;
    for &x in xs {
        r = r.wrapping_add(x).wrapping_mul(17);
    }
    r
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let line = lines.next().context("empty input")??;
    ensure!(lines.next().is_none(), "extra input");
    let steps = line.split(',').collect::<Vec<_>>();
    let mut boxes: [Vec<(&str, i64)>; 256] = std::array::from_fn(|_| Vec::new());
    for step in steps {
        match (step.split_once('='), step.strip_suffix('-')) {
            (Some((label, focal)), None) => {
                let focal = focal.parse()?;
                let i = hash(label.as_bytes()) as usize;
                match boxes[i].iter_mut().find(|(x, _)| *x == label) {
                    Some(x) => x.1 = focal,
                    None => boxes[i].push((label, focal)),
                }
            }
            (None, Some(label)) => {
                let i = hash(label.as_bytes()) as usize;
                boxes[i].retain(|(x, _)| *x != label);
            }
            _ => bail!("invalid step {step:?}"),
        }
    }
    let total = (boxes.into_iter().zip(1 ..))
        .flat_map(|(xs, i)| xs.into_iter().zip(1 ..).map(move |((_, x), j)| i * j * x))
        .sum::<i64>();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "212449\n");
