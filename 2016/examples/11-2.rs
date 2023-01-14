use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use regex::Regex;

type Name = internment::Intern<String>;

fn all<'a>(
    floor: usize, gs: &'a [BTreeSet<Name>], ms: &'a [BTreeSet<Name>],
) -> impl Iterator<Item = (bool, Name)> + 'a {
    gs[floor].iter().map(|&x| (true, x)).chain(ms[floor].iter().map(|&x| (false, x)))
}

fn update(
    floor: usize, next_floor: usize, gs: &[BTreeSet<Name>], ms: &[BTreeSet<Name>], g: bool, x: Name,
) -> (Vec<BTreeSet<Name>>, Vec<BTreeSet<Name>>) {
    let mut gs = gs.to_vec();
    let mut ms = ms.to_vec();
    let xs = if g { &mut gs } else { &mut ms };
    assert!(xs[floor].remove(&x));
    assert!(xs[next_floor].insert(x));
    (gs, ms)
}

fn canon(gs: &[BTreeSet<Name>], ms: &[BTreeSet<Name>]) -> [BTreeSet<usize>; 8] {
    let mut r = std::array::from_fn(|_| BTreeSet::new());
    let mut ns = HashMap::new();
    let mut get = |x: Name| -> usize {
        let n = ns.len();
        match ns.entry(x) {
            Entry::Occupied(x) => *x.get(),
            Entry::Vacant(x) => *x.insert(n),
        }
    };
    for i in 0 .. 4 {
        for &x in &gs[i] {
            r[2 * i].insert(get(x));
        }
        for &x in &ms[i] {
            r[2 * i + 1].insert(get(x));
        }
    }
    r
}

fn valid(gs: &[BTreeSet<Name>], ms: &[BTreeSet<Name>]) -> bool {
    (0 .. 4).all(|i| gs[i].is_empty() || ms[i].iter().all(|x| gs[i].contains(x)))
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut gs = Vec::<BTreeSet<Name>>::new();
    let mut ms = Vec::<BTreeSet<Name>>::new();
    let gr = Regex::new("a ([^ ]*) generator")?;
    let mr = Regex::new("a ([^ ]*)-compatible microchip")?;
    for line in BufReader::new(input).lines() {
        let line = line?;
        gs.push(gr.captures_iter(&line).map(|x| Name::from_ref(&x[1])).collect());
        ms.push(mr.captures_iter(&line).map(|x| Name::from_ref(&x[1])).collect());
    }
    assert_eq!(gs.len(), 4);
    for component in ["elerium", "dilithium"] {
        gs[0].insert(Name::from_ref(component));
        ms[0].insert(Name::from_ref(component));
    }
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), 0, gs, ms));
    let mut visited = HashSet::new();
    while let Some((Reverse(steps), floor, gs, ms)) = todo.pop() {
        if !visited.insert((floor, canon(&gs, &ms))) {
            continue;
        }
        if (0 .. 3).all(|i| gs[i].is_empty() && ms[i].is_empty()) {
            assert_eq!(floor, 3);
            writeln!(output, "{steps}")?;
            break;
        }
        let steps = steps + 1;
        let mut next_floors = Vec::new();
        if floor > 0 && !(0 .. floor).all(|i| gs[i].is_empty() && ms[i].is_empty()) {
            next_floors.push(floor - 1);
        }
        if floor < 3 {
            next_floors.push(floor + 1);
        }
        for next_floor in next_floors {
            for (g, x) in all(floor, &gs, &ms) {
                let (gs, ms) = update(floor, next_floor, &gs, &ms, g, x);
                if valid(&gs, &ms) {
                    todo.push((Reverse(steps), next_floor, gs.clone(), ms.clone()));
                }
                if next_floor < floor {
                    continue;
                }
                for (g, x) in all(floor, &gs, &ms) {
                    let (gs, ms) = update(floor, next_floor, &gs, &ms, g, x);
                    if valid(&gs, &ms) {
                        todo.push((Reverse(steps), next_floor, gs, ms));
                    }
                }
            }
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "71\n");
