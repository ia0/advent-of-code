use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn conv(x: &str) -> [u8; 2] {
    x.as_bytes().try_into().unwrap()
}

fn next(
    graph: &HashMap<[u8; 2], HashSet<[u8; 2]>>, prev: &HashSet<Vec<[u8; 2]>>,
) -> HashSet<Vec<[u8; 2]>> {
    let mut next = HashSet::new();
    for xs in prev {
        let (x, ys) = xs.split_first().unwrap();
        for z in graph.get(x).unwrap() {
            if ys.iter().all(|y| graph.get(y).unwrap().contains(z)) {
                let mut zs = xs.to_vec();
                zs.push(*z);
                zs.sort();
                next.insert(zs);
            }
        }
    }
    next
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut graph = HashMap::<[u8; 2], HashSet<[u8; 2]>>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (a, b) = line.split_once("-").unwrap();
        let a = conv(a);
        let b = conv(b);
        assert!(a != b);
        assert!(graph.entry(a).or_default().insert(b));
        assert!(graph.entry(b).or_default().insert(a));
    }
    let mut prev = graph.keys().map(|x| vec![*x]).collect();
    loop {
        let cur = next(&graph, &prev);
        if cur.is_empty() {
            assert_eq!(prev.len(), 1);
            let xs = prev.iter().next().unwrap();
            let xs: Vec<_> = xs.iter().map(|x| std::str::from_utf8(x).unwrap()).collect();
            writeln!(output, "{}", xs.join(","))?;
            break;
        }
        prev = cur;
    }
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "ah,ap,ek,fj,fr,jt,ka,ln,me,mp,qa,ql,zg\n");
