extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use adventofcode::binary_search;
use regex::Regex;

fn satisfy(
    deps: &HashMap<String, (usize, HashMap<String, usize>)>, mut avail: HashMap<String, i64>,
) -> bool {
    while avail["ORE"] >= 0 {
        let next = match avail.iter().find(|(_, &x)| x < 0).map(|(k, _)| k) {
            None => return true,
            Some(next) => next.clone(),
        };
        let count = -avail[&next] as usize;
        let (step, args) = deps.get(&next).unwrap();
        let iter = (count + step - 1) / step;
        for (arg_k, &arg_v) in args {
            *avail.entry(arg_k.clone()).or_default() -= (iter * arg_v) as i64;
        }
        *avail.get_mut(&next).unwrap() += (iter * step) as i64;
    }
    false
}

fn main() {
    let file = File::open("examples/14.txt").unwrap();
    let regex = Regex::new("([0-9]+) ([A-Z]+)").unwrap();
    let mut deps: HashMap<String, (usize, HashMap<String, usize>)> = HashMap::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut elements: Vec<(usize, String)> =
            regex.captures_iter(&line).map(|c| (c[1].parse().unwrap(), c[2].to_string())).collect();
        let (count, output) = elements.pop().unwrap();
        let mut inputs = HashMap::new();
        for (count, input) in elements {
            assert!(inputs.insert(input.clone(), count).is_none());
        }
        assert!(deps.insert(output, (count, inputs)).is_none());
    }
    let mut avail = HashMap::new();
    avail.insert("ORE".to_string(), 1000000000000);
    println!(
        "{}",
        binary_search(|fuel| {
            let mut avail = avail.clone();
            avail.insert("FUEL".to_string(), -(fuel as i64));
            !satisfy(&deps, avail)
        }) - 1
    );
}
