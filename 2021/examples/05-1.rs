use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use adventofcode::Coord;

struct Line {
    a: Coord<usize>,
    b: Coord<usize>,
}

fn parse_coord(input: &str) -> Coord<usize> {
    let words: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    assert_eq!(words.len(), 2);
    Coord { x: words[0], y: words[1] }
}

fn parse_line(input: &str) -> Line {
    let words: Vec<_> = input.split_whitespace().collect();
    assert_eq!(words.len(), 3);
    assert_eq!(words[1], "->");
    let a = parse_coord(words[0]);
    let b = parse_coord(words[2]);
    Line { a, b }
}

fn range(x: usize, y: usize) -> impl Iterator<Item = usize> {
    std::cmp::min(x, y) ..= std::cmp::max(x, y)
}

fn main() {
    let input = File::open("examples/05.txt").unwrap();
    let lines: Vec<_> = BufReader::new(input).lines().map(|x| parse_line(&x.unwrap())).collect();
    let mut count: HashMap<_, usize> = HashMap::new();
    for line in lines {
        let points: Box<dyn Iterator<Item = Coord<usize>>> = if line.a.x == line.b.x {
            Box::new(range(line.a.y, line.b.y).map(|y| Coord { x: line.a.x, y }))
        } else if line.a.y == line.b.y {
            Box::new(range(line.a.x, line.b.x).map(|x| Coord { x, y: line.a.y }))
        } else {
            continue;
        };
        for point in points {
            *count.entry(point).or_default() += 1;
        }
    }
    println!("{}", count.values().filter(|&&x| x > 1).count());
}
