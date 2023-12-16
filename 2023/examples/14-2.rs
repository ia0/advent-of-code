use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Result};

fn roll(map: &mut [Vec<u8>]) {
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
}

fn roll4(map: &mut Vec<Vec<u8>>) {
    for _ in 0 .. 4 {
        roll(map);
        rotate(map);
    }
}

fn load(map: &[Vec<u8>]) -> usize {
    let n = map.len();
    (0 .. n).map(|i| map[i].iter().filter(|&&x| x == b'O').count() * (n - i)).sum::<usize>()
}

fn rotate(map: &mut Vec<Vec<u8>>) {
    *map =
        (0 .. map[0].len()).map(|j| (0 .. map.len()).rev().map(|i| map[i][j]).collect()).collect();
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let lines = BufReader::new(input).lines();
    let mut map = lines.map(|x| Ok(x?.into_bytes())).collect::<Result<Vec<_>>>()?;
    ensure!(!map.is_empty());
    ensure!(map.iter().all(|x| x.len() == map[0].len()));
    ensure!(map.iter().all(|x| x.iter().all(|x| matches!(x, b'.' | b'#' | b'O'))));
    let mut visited = HashMap::new();
    for i in 1 .. {
        roll4(&mut map);
        if let Some(k) = visited.insert(map.clone(), i) {
            for _ in 0 .. (1_000_000_000 - i) % (i - k) {
                roll4(&mut map);
            }
            writeln!(output, "{}", load(&map))?;
            return Ok(());
        }
    }
    unreachable!()
}

adventofcode::main!(solve("examples/14.txt") == "83516\n");
