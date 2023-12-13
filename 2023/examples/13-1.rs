use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Result};

fn transpose(map: &mut Vec<Vec<u8>>) {
    *map = (0 .. map[0].len()).map(|j| (0 .. map.len()).map(|i| map[i][j]).collect()).collect();
}

fn count(map: &[Vec<u8>]) -> i64 {
    (1 .. map.len()).filter(|&i| reflect(map, i)).sum::<usize>() as i64
}

fn reflect(map: &[Vec<u8>], mid: usize) -> bool {
    map[mid ..].iter().zip(map[.. mid].iter().rev()).all(|(x, y)| x == y)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut maps = Vec::new();
    let mut done = false;
    while !done {
        done = true;
        let mut map = Vec::<Vec<u8>>::new();
        for line in &mut lines {
            let line = line?;
            if line.is_empty() {
                done = false;
                break;
            }
            map.push(line.into_bytes());
        }
        ensure!(!map.is_empty());
        ensure!(map.iter().all(|row| row.len() == map[0].len()));
        maps.push(map);
    }
    let mut total = 0;
    for mut map in maps {
        total += 100 * count(&map);
        transpose(&mut map);
        total += count(&map);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/13.txt") == "37718\n");
