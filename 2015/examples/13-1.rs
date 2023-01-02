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
    graph: &HashMap<Name, HashMap<Name, i64>>, cost: i64, first: Name, pos: Name,
    rem: HashSet<Name>,
) -> i64 {
    next(&rem)
        .map(|(next, rem)| {
            find(graph, cost + graph[&pos][&next] + graph[&next][&pos], first, next, rem)
        })
        .max()
        .unwrap_or_else(|| cost + graph[&first][&pos] + graph[&pos][&first])
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let regex = Regex::new(r"^(.*) would (.*) (.*) happiness units by sitting next to (.*)\.$")?;
    let mut graph = HashMap::<Name, HashMap<Name, i64>>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let capture = regex.captures(&line).unwrap();
        let a = Intern::from_ref(&capture[1]);
        let b = Intern::from_ref(&capture[4]);
        let mut d: i64 = capture[3].parse()?;
        if &capture[2] == "lose" {
            d = -d;
        }
        assert!(graph.entry(a).or_default().insert(b, d).is_none());
    }
    let (first, names) = next(&graph.keys().cloned().collect()).next().unwrap();
    writeln!(output, "{}", find(&graph, 0, first, first, names))?;
    Ok(())
}

adventofcode::main!(solve("examples/13.txt") == "664\n");
