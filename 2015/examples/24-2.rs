use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use number_encoding::combinadics;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let weights = (BufReader::new(input).lines())
        .map(|x| Result::Ok(x?.parse()?))
        .collect::<Result<Vec<usize>>>()?;
    assert!(weights.len() <= 32);
    let sum = weights.iter().sum::<usize>();
    assert!(sum % 4 == 0);
    let target = sum / 4;
    for len in 1 ..= weights.len() {
        let mut best = None;
        let mut iter = combinadics::Iter::new(len);
        for _ in 0 .. number_encoding::combination(weights.len(), len) {
            let xs = iter.get().iter().map(|&i| weights[i]);
            if xs.clone().sum::<usize>() == target {
                let cur = xs.product::<usize>();
                if best.is_none_or(|best| cur < best) {
                    best = Some(cur);
                }
            }
            iter.advance();
        }
        if let Some(best) = best {
            writeln!(output, "{best}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/24.txt") == "74850409\n");
