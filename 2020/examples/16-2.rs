use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::ops::Range;

use regex::Regex;

struct Field {
    name: String,
    ranges: [Range<usize>; 2],
}

impl Field {
    fn is_valid(&self, x: usize) -> bool {
        self.ranges[0].contains(&x) || self.ranges[1].contains(&x)
    }
}

fn parse_ticket(x: &str) -> Vec<usize> {
    x.split(',').map(|x| x.parse().unwrap()).collect()
}

fn solve(input: &HashMap<usize, HashSet<usize>>, output: &mut HashMap<usize, usize>) -> bool {
    match input.iter().filter(|(k, _)| !output.contains_key(k)).min_by_key(|(_, vs)| vs.len()) {
        None => true,
        Some((&k, vs)) => {
            for &v in vs {
                if output.values().find(|&&x| x == v).is_some() {
                    continue;
                }
                assert!(output.insert(k, v).is_none());
                if solve(input, output) {
                    return true;
                }
                assert!(output.remove(&k).is_some());
            }
            false
        }
    }
}

fn main() {
    let input = File::open("examples/16.txt").unwrap();
    let regex = Regex::new(r#"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$"#).unwrap();
    let mut fields = Vec::new();
    let mut lines = BufReader::new(input).lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let captures = regex.captures(&line).unwrap();
        fields.push(Field {
            name: captures.get(1).unwrap().as_str().to_string(),
            ranges: [
                Range {
                    start: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    end: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() + 1,
                },
                Range {
                    start: captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    end: captures.get(5).unwrap().as_str().parse::<usize>().unwrap() + 1,
                },
            ],
        });
    }
    let n = fields.len();
    assert_eq!(lines.next().unwrap().unwrap(), "your ticket:");
    let ticket = parse_ticket(&lines.next().unwrap().unwrap());
    assert_eq!(lines.next().unwrap().unwrap(), "");
    assert_eq!(lines.next().unwrap().unwrap(), "nearby tickets:");
    let mut nearby = Vec::new();
    for line in lines {
        nearby.push(parse_ticket(&line.unwrap()));
        assert_eq!(nearby.last().unwrap().len(), n);
    }
    nearby.retain(|ticket| {
        ticket.iter().all(|&value| fields.iter().any(|field| field.is_valid(value)))
    });
    let mut edges: HashMap<usize, HashSet<usize>> = HashMap::new();
    for f in 0 .. n {
        assert!(edges
            .insert(
                f,
                HashSet::from_iter(
                    (0 .. n).filter(|&i| nearby.iter().all(|x| fields[f].is_valid(x[i])))
                )
            )
            .is_none());
    }
    let mut order = HashMap::new();
    assert!(solve(&edges, &mut order));
    let mut product = 1;
    for f in 0 .. n {
        if fields[f].name.starts_with("departure ") {
            product *= ticket[order[&f]];
        }
    }
    println!("{}", product);
}
