use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Rule {
    Leaf(u8),
    Node(Vec<Vec<usize>>),
}

fn add(output: &mut HashSet<String>, input: &[&HashSet<String>]) {
    let mut stack: Vec<_> = input.iter().map(|x| x.iter().peekable()).collect();
    loop {
        let mut x = String::new();
        for y in &mut stack {
            x.push_str(y.peek().unwrap());
        }
        output.insert(x);
        stack.last_mut().unwrap().next();
        while stack.last_mut().unwrap().peek().is_none() {
            stack.pop();
            if stack.is_empty() {
                return;
            }
            stack.last_mut().unwrap().next();
        }
        for i in stack.len() .. input.len() {
            stack.push(input[i].iter().peekable());
        }
    }
}

fn compute(mut rules: HashMap<usize, Rule>) -> HashSet<String> {
    let mut messages = HashMap::new();
    while !rules.is_empty() {
        let (&next, _) = rules
            .iter()
            .find(|(_, v)| match v {
                Rule::Leaf(_) => true,
                Rule::Node(v) => {
                    v.iter().map(|x| x.iter()).flatten().all(|x| messages.contains_key(x))
                }
            })
            .unwrap();
        let mut all = HashSet::new();
        match rules.remove(&next).unwrap() {
            Rule::Leaf(x) => {
                all.insert(String::from_utf8(vec![x]).unwrap());
            }
            Rule::Node(v) => {
                for v in v {
                    let xs: Vec<_> = v.iter().map(|x| messages.get(x).unwrap()).collect();
                    add(&mut all, &xs);
                }
            }
        };
        assert!(messages.insert(next, all).is_none());
    }
    messages.remove(&0).unwrap()
}

fn main() {
    let input = File::open("examples/19.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut rules = HashMap::new();
    for line in &mut lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let name: usize = words[0].strip_suffix(':').unwrap().parse().unwrap();
        let rule = match words[1].strip_prefix('"') {
            None => {
                let mut rule = Vec::new();
                let mut branch = Vec::new();
                for word in &words[1 ..] {
                    if *word == "|" {
                        rule.push(branch);
                        branch = Vec::new();
                    } else {
                        branch.push(word.parse().unwrap());
                    }
                }
                rule.push(branch);
                Rule::Node(rule)
            }
            Some(mut word) => {
                word = word.strip_suffix('"').unwrap();
                assert_eq!(word.len(), 1);
                Rule::Leaf(word.as_bytes()[0])
            }
        };
        assert!(rules.insert(name, rule).is_none());
    }
    let messages: Vec<_> = lines.map(|x| x.unwrap()).collect();
    let all = compute(rules);
    println!("{}", messages.iter().filter(|&m| all.contains(m)).count());
}
