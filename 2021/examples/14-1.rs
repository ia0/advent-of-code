use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/14.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut polymer = lines.next().unwrap().unwrap().into_bytes();
    assert!(lines.next().unwrap().unwrap().is_empty());
    let mut rules = HashMap::new();
    for line in lines {
        let line = line.unwrap().into_bytes();
        assert_eq!(line.len(), 7);
        assert_eq!(&line[2 .. 6], b" -> ");
        assert!(rules.insert((line[0], line[1]), line[6]).is_none());
    }
    for _ in 0 .. 10 {
        let mut next = vec![polymer[0]];
        for i in 1 .. polymer.len() {
            match rules.get(&(polymer[i - 1], polymer[i])) {
                None => (),
                Some(&x) => next.push(x),
            }
            next.push(polymer[i]);
        }
        polymer = next;
    }
    let mut count = HashMap::new();
    for x in polymer {
        *count.entry(x).or_insert(0usize) += 1;
    }
    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();
    println!("{}", max - min);
}
