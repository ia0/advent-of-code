use std::collections::HashMap;
use std::io::BufRead;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() {
    let mut fabric = HashMap::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let words: Vec<_> = line
            .split(|x| ['n', '#', ' ', '@', ',', ':', 'x'].contains(&x))
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(words.len(), 5);
        for x in words[1] .. words[1] + words[3] {
            for y in words[2] .. words[2] + words[4] {
                *fabric.entry(Coord { x, y }).or_insert(0) += 1;
            }
        }
    }
    let mut count = 0;
    for &overlap in fabric.values() {
        count += (overlap > 1) as usize;
    }
    println!("{}", count);
}
