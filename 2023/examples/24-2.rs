use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::egcd;
use anyhow::{ensure, Context, Result};
use num::{BigInt, One, Zero};

fn vector(input: &str) -> Result<[BigInt; 3]> {
    let xs = input.split(", ").map(|x| Ok(x.trim().parse()?)).collect::<Result<Vec<BigInt>>>()?;
    ensure!(xs.iter().all(|x| !x.is_zero()));
    xs.try_into().ok().context("bad vector")
}

#[derive(Debug)]
struct Hail {
    pos: [BigInt; 3],
    vel: [BigInt; 3],
}

fn reduce(matrix: &mut [[BigInt; 7]], k: usize) -> Option<()> {
    let n = matrix.len();
    if n <= k {
        return None;
    }
    if matrix[k][k].is_zero() {
        let i = (k + 1 .. n).find(|&i| !matrix[i][k].is_zero())?;
        matrix.swap(k, i);
    }
    for i in k + 1 .. n {
        let (_, s, t) = egcd(matrix[k][k].clone(), matrix[i][k].clone());
        for j in k .. 7 {
            matrix[k][j] = s.clone() * matrix[k][j].clone() + t.clone() * matrix[i][j].clone();
        }
        let r = matrix[i][k].clone() / matrix[k][k].clone();
        for j in k .. 7 {
            matrix[i][j] -= r.clone() * matrix[k][j].clone();
        }
    }
    Some(())
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut hails = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (pos, vel) = line.split_once(" @ ").context("no @")?;
        hails.push(Hail { pos: vector(pos)?, vel: vector(vel)? });
    }
    let mut matrix = Vec::new();
    let mut rows = (1 .. hails.len())
        .flat_map(|b| (0 .. b).flat_map(move |a| [(a, b, 0, 1), (a, b, 0, 2), (a, b, 1, 2)]))
        .map(|(a, b, i, j)| {
            let mut row = std::array::from_fn(|_| BigInt::zero());
            row[i] = hails[b].vel[j].clone() - hails[a].vel[j].clone();
            row[j] = hails[a].vel[i].clone() - hails[b].vel[i].clone();
            row[3 + i] = hails[a].pos[j].clone() - hails[b].pos[j].clone();
            row[3 + j] = hails[b].pos[i].clone() - hails[a].pos[i].clone();
            row[6] = hails[b].pos[i].clone() * hails[b].vel[j].clone()
                - hails[a].pos[i].clone() * hails[a].vel[j].clone();
            row[6] += hails[a].pos[j].clone() * hails[a].vel[i].clone()
                - hails[b].pos[j].clone() * hails[b].vel[i].clone();
            row
        });
    let mut k = 0;
    while k < 6 {
        while reduce(&mut matrix, k).is_none() {
            matrix.retain(|x| !x.iter().all(|x| x.is_zero()));
            matrix.push(rows.next().unwrap());
            k = 0;
        }
        k += 1;
    }
    assert_eq!(matrix.len(), 6);
    for k in (0 .. 6).rev() {
        assert!((matrix[k][6].clone() % matrix[k][k].clone()).is_zero());
        let r = matrix[k][k].clone();
        matrix[k][6] /= r;
        matrix[k][k] = BigInt::one();
        for i in 0 .. k {
            let r = matrix[i][k].clone() * matrix[k][6].clone();
            matrix[i][6] -= r;
            matrix[i][k] = BigInt::zero();
        }
    }
    writeln!(output, "{}", (0 .. 3).map(|i| &matrix[i][6]).sum::<BigInt>())?;
    Ok(())
}

adventofcode::main!(solve("examples/24.txt") == "695832176624149\n");
