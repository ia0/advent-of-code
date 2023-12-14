use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let lines = BufReader::new(input).lines();
    let mut map = lines.map(|x| Ok(x?.into_bytes())).collect::<Result<Vec<_>>>()?;
    ensure!(!map.is_empty());
    ensure!(map.iter().all(|x| x.len() == map[0].len()));
    ensure!(map.iter().all(|x| x.iter().all(|x| matches!(x, b'.' | b'#' | b'O'))));
    for i in 0 .. map[0].len() {
        let mut k = 0;
        for j in 0 .. map.len() {
            match map[j][i] {
                b'.' => (),
                b'O' => {
                    if k < j {
                        assert_eq!(std::mem::replace(&mut map[k][i], b'O'), b'.');
                        map[j][i] = b'.';
                    }
                    k += 1;
                }
                b'#' => k = j + 1,
                _ => unreachable!(),
            }
        }
    }
    let n = map.len();
    let total =
        (0 .. n).map(|i| map[i].iter().filter(|&&x| x == b'O').count() * (n - i)).sum::<usize>();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/14.txt") == "110821\n");
