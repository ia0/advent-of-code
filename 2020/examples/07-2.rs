use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn count(graph: &HashMap<String, HashMap<String, i32>>, bag: &str) -> i32 {
    let mut total = 1;
    for (bag, n) in graph.get(bag).iter().map(|x| x.iter()).flatten() {
        total += n * count(graph, bag);
    }
    total
}

fn main() {
    let input = File::open("examples/07.txt").unwrap();
    let mut graph: HashMap<String, HashMap<String, i32>> = HashMap::new();
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
            assert!(inners
                .insert(
                    captures.get(2).unwrap().as_str().into(),
                    captures.get(1).unwrap().as_str().parse().unwrap()
                )
                .is_none());
        }
        assert!(graph.insert(outer, inners).is_none());
    }
    println!("{}", count(&graph, "shiny gold") - 1);
}
