use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let input = File::open("examples/07.txt").unwrap();
    let mut graph: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut rev_graph: HashMap<String, HashSet<String>> = HashMap::new();
    let outer_regex = Regex::new("(.+) bags contain").unwrap();
    let inner_regex = Regex::new("([0-9]+) ([^,]+) bag").unwrap();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        if let Some(outer) = line.strip_suffix(" bags contain no other bags.") {
            assert!(graph.insert(outer.into(), HashMap::new()).is_none());
            continue;
        }
        let captures = outer_regex.captures(&line).unwrap();
        let outer: String = captures.get(1).unwrap().as_str().into();
        let mut inners = HashMap::new();
        for captures in inner_regex.captures_iter(&line) {
            let inner: String = captures.get(2).unwrap().as_str().into();
            assert!(inners
                .insert(inner.clone(), captures.get(1).unwrap().as_str().parse().unwrap())
                .is_none());
            assert!(rev_graph.entry(inner).or_default().insert(outer.clone()));
        }
        assert!(graph.insert(outer, inners).is_none());
    }
    let mut visited = HashSet::new();
    let mut todo = Vec::new();
    todo.push("shiny gold");
    while let Some(bag) = todo.pop() {
        if !visited.insert(bag) {
            continue;
        }
        for bag in rev_graph.get(bag).iter().map(|x| x.iter()).flatten() {
            todo.push(bag);
        }
    }
    println!("{}", visited.len() - 1);
}
