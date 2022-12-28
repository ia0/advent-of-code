use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn link(map: &mut HashMap<String, HashSet<String>>, x: String, y: String) {
    assert!(map.entry(x.clone()).or_default().insert(y.clone()));
    assert!(map.entry(y).or_default().insert(x));
}

fn small(x: &str) -> bool {
    x.chars().next().unwrap().is_lowercase()
}

fn count(map: &HashMap<String, HashSet<String>>) -> usize {
    struct Frame<'a> {
        pos: &'a str,
        seen: HashSet<&'a str>,
        ext: Option<&'a str>,
    }
    let mut count = 0;
    let mut todo = vec![Frame { pos: "start", seen: HashSet::new(), ext: None }];
    while let Some(Frame { pos, mut seen, mut ext }) = todo.pop() {
        if pos == "end" {
            count += 1;
            continue;
        }
        if small(pos) && !seen.insert(pos) {
            if ext.is_some() || pos == "start" {
                continue;
            }
            ext = Some(pos);
        }
        let nexts = match map.get(pos) {
            None => continue,
            Some(x) => x,
        };
        for next in nexts {
            todo.push(Frame { pos: next, seen: seen.clone(), ext });
        }
    }
    count
}

fn main() {
    let input = File::open("examples/12.txt").unwrap();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split('-').collect();
        assert_eq!(words.len(), 2);
        link(&mut map, words[0].to_string(), words[1].to_string());
    }
    println!("{}", count(&map));
}
