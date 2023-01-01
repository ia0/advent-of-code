use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use internment::Intern;
use regex::Regex;

type Name = Intern<String>;

fn next(xs: &HashSet<Name>) -> impl Iterator<Item = (Name, HashSet<Name>)> + '_ {
    xs.iter().map(|&x| {
        let mut ys = xs.clone();
        assert!(ys.remove(&x));
        (x, ys)
    })
}

fn find(
    graph: &HashMap<Name, HashMap<Name, usize>>, cost: usize, pos: Name, rem: HashSet<Name>,
) -> usize {
    next(&rem)
        .map(|(next, rem)| find(graph, cost + graph[&pos][&next], next, rem))
        .max()
        .unwrap_or(cost)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let regex = Regex::new("^(.*) to (.*) = (.*)$")?;
    let mut graph = HashMap::<Name, HashMap<Name, usize>>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let capture = regex.captures(&line).unwrap();
        let a = Intern::from_ref(&capture[1]);
        let b = Intern::from_ref(&capture[2]);
        let d = capture[3].parse()?;
        assert!(graph.entry(a).or_default().insert(b, d).is_none());
        assert!(graph.entry(b).or_default().insert(a, d).is_none());
    }
    let names = graph.keys().cloned().collect::<HashSet<_>>();
    writeln!(
        output,
        "{}",
        next(&names).map(|(pos, rem)| find(&graph, 0, pos, rem)).max().unwrap()
    )?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "909\n");
