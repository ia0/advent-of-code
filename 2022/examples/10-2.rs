use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{bail, ensure, Context, Error, Result};

enum Instruction {
    AddX(i64),
    NoOp,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut words = input.split_whitespace();
        let instr = match words.next().context("missing opcode")? {
            "noop" => Instruction::NoOp,
            "addx" => Instruction::AddX(words.next().context("missing argument")?.parse()?),
            _ => bail!("invalid opcode"),
        };
        ensure!(words.next().is_none(), "extra arguments");
        Ok(instr)
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut instrs = Vec::new();
    for line in BufReader::new(input).lines() {
        instrs.push(line?.parse::<Instruction>()?);
    }
    let mut trace = vec![1];
    for instr in instrs {
        let reg = *trace.last().unwrap();
        match instr {
            Instruction::AddX(diff) => trace.extend([reg, reg + diff]),
            Instruction::NoOp => trace.push(reg),
        }
    }
    for row in 0 .. 6 {
        for col in 0 .. 40 {
            let pixel = match trace[40 * row + col] - col as i64 {
                -1 | 0 | 1 => '#',
                _ => '.',
            };
            write!(output, "{pixel}")?;
        }
        writeln!(output)?;
    }
    Ok(())
}

adventofcode::main!(
    solve("examples/10.txt")
        == "\
####.####.#..#..##..#....###...##..###..
#....#....#..#.#..#.#....#..#.#..#.#..#.
###..###..#..#.#....#....#..#.#..#.#..#.
#....#....#..#.#.##.#....###..####.###..
#....#....#..#.#..#.#....#....#..#.#....
####.#.....##...###.####.#....#..#.#....
"
);
