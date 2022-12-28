use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Count(HashMap<u8, usize>);

impl Count {
    fn incr(&mut self, x: u8) {
        *self.0.entry(x).or_default() += 1;
    }

    fn merge(&mut self, other: Option<&Count>) {
        let other = match other {
            None => return,
            Some(x) => x,
        };
        for (&x, &y) in &other.0 {
            *self.0.entry(x).or_default() += y;
        }
    }
}

fn main() {
    let input = File::open("examples/14.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let polymer = lines.next().unwrap().unwrap().into_bytes();
    assert!(lines.next().unwrap().unwrap().is_empty());
    let mut rules = HashMap::new();
    for line in lines {
        let line = line.unwrap().into_bytes();
        assert_eq!(line.len(), 7);
        assert_eq!(&line[2 .. 6], b" -> ");
        assert!(rules.insert((line[0], line[1]), line[6]).is_none());
    }
    let mut cache: HashMap<(u8, u8), Count> = HashMap::new();
    for _ in 0 .. 40 {
        cache = rules
            .iter()
            .map(|(&(a, b), &c)| {
                let mut z = Count::default();
                z.merge(cache.get(&(a, c)));
                z.incr(c);
                z.merge(cache.get(&(c, b)));
                ((a, b), z)
            })
            .collect();
    }
    let mut count = Count::default();
    count.incr(polymer[0]);
    for i in 1 .. polymer.len() {
        count.merge(Some(&cache[&(polymer[i - 1], polymer[i])]));
        count.incr(polymer[i]);
    }
    let max = count.0.values().max().unwrap();
    let min = count.0.values().min().unwrap();
    println!("{}", max - min);
}
