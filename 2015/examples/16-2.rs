use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use internment::Intern;

type Name = Intern<String>;

fn included(xs: &HashMap<Name, usize>, ys: &HashMap<Name, usize>) -> bool {
    xs.iter().all(|(x, v)| {
        ys.get(x).is_some_and(|w| match x.as_str() {
            "cats" | "trees" => v > w,
            "pomeranians" | "goldfish" => v < w,
            _ => w == v,
        })
    })
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let gift = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .map(|&(x, y)| (Name::from_ref(x), y))
    .collect::<HashMap<Name, usize>>();
    let mut sues = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (sue, line) = line.split_once(": ").unwrap();
        sues.push(
            line.split(", ")
                .map(|x| {
                    let (x, y) = x.split_once(": ").unwrap();
                    (Name::from_ref(x), y.parse().unwrap())
                })
                .collect::<HashMap<Name, usize>>(),
        );
        assert_eq!(sue, format!("Sue {}", sues.len()));
    }
    let candidates = sues
        .iter()
        .zip(1 ..)
        .filter_map(|(x, y)| included(x, &gift).then_some(y))
        .collect::<Vec<_>>();
    assert_eq!(candidates.len(), 1);
    writeln!(output, "{}", candidates[0])?;
    Ok(())
}

adventofcode::main!(solve("examples/16.txt") == "323\n");
