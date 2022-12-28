use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<usize>;

struct Line {
    a: Coord,
    b: Coord,
}

fn parse_coord(input: &str) -> Coord {
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

fn iter(a: usize, b: usize) -> Result<Box<dyn Iterator<Item = usize>>, usize> {
    use std::cmp::Ordering::*;
    match a.cmp(&b) {
        Less => Ok(Box::new(a ..= b)),
        Equal => Err(a),
        Greater => Ok(Box::new((b ..= a).rev())),
    }
}

fn main() {
    let input = File::open("examples/05.txt").unwrap();
    let lines: Vec<_> = BufReader::new(input).lines().map(|x| parse_line(&x.unwrap())).collect();
    let mut count: HashMap<_, usize> = HashMap::new();
    for line in lines {
        let xs = iter(line.a.x, line.b.x);
        let ys = iter(line.a.y, line.b.y);
        let points: Box<dyn Iterator<Item = Coord>> = match (xs, ys) {
            (Err(x), Err(y)) => Box::new(std::iter::once(Coord { x, y })),
            (Ok(xs), Err(y)) => Box::new(xs.map(move |x| Coord { x, y })),
            (Err(x), Ok(ys)) => Box::new(ys.map(move |y| Coord { x, y })),
            (Ok(xs), Ok(ys)) => Box::new(xs.zip(ys).map(|(x, y)| Coord { x, y })),
        };
        for point in points {
            *count.entry(point).or_default() += 1;
        }
    }
    println!("{}", count.values().filter(|&&x| x > 1).count());
}
