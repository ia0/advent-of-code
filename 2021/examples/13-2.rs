use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(i64),
    Y(i64),
}

impl Fold {
    fn apply(self, p: Coord) -> Coord {
        let m = |x, r| std::cmp::min(x, 2 * r - x);
        match self {
            Fold::X(x) => Coord { x: m(p.x, x), ..p },
            Fold::Y(y) => Coord { y: m(p.y, y), ..p },
        }
    }

    fn contains(self, p: Coord) -> bool {
        match self {
            Fold::X(x) => p.x == x,
            Fold::Y(y) => p.y == y,
        }
    }
}

fn fold(holes: &mut HashSet<Coord>, fold: Fold) {
    *holes = holes.drain().map(|p| fold.apply(p)).collect();
    holes.retain(|&p| !fold.contains(p));
}

fn main() {
    let input = File::open("examples/13.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut holes = HashSet::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let xy: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(xy.len(), 2);
        assert!(holes.insert(Coord { x: xy[0], y: xy[1] }));
    }
    for line in lines {
        let line = line.unwrap();
        let words: Vec<_> = line.split('=').collect();
        assert_eq!(words.len(), 2);
        let z = words[1].parse().unwrap();
        fold(
            &mut holes,
            match words[0] {
                "fold along x" => Fold::X(z),
                "fold along y" => Fold::Y(z),
                _ => unreachable!(),
            },
        );
    }
    adventofcode::print_set(&holes, false);
}
