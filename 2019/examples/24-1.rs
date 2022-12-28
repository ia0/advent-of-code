use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Map(u32);

impl Map {
    fn add(self, x: i8, y: i8) -> Map {
        Map(self.0 | 1 << 5 * y + x)
    }

    fn remove(self, x: i8, y: i8) -> Map {
        Map(self.0 & !(1 << 5 * y + x))
    }

    fn bug(self, x: i8, y: i8) -> bool {
        0 <= x && x < 5 && 0 <= y && y < 5 && (self.0 & 1 << 5 * y + x != 0)
    }

    fn bugs(self, x: i8, y: i8) -> usize {
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .filter(|&&(x, y)| self.bug(x, y))
            .count()
    }

    fn step(self) -> Map {
        let mut r = self;
        for y in 0 .. 5 {
            for x in 0 .. 5 {
                if self.bug(x, y) {
                    if self.bugs(x, y) != 1 {
                        r = r.remove(x, y);
                    }
                } else {
                    let bugs = self.bugs(x, y);
                    if bugs == 1 || bugs == 2 {
                        r = r.add(x, y);
                    }
                }
            }
        }
        r
    }
}

fn main() {
    let input = File::open("examples/24.txt").unwrap();
    let mut map = Map::default();
    for (y, line) in BufReader::new(input).lines().enumerate() {
        for (x, c) in line.unwrap().into_bytes().iter().enumerate() {
            match c {
                b'.' => (),
                b'#' => map = map.add(x as i8, y as i8),
                _ => panic!(),
            }
        }
    }
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(map) {
            println!("{}", map.0);
            return;
        }
        map = map.step();
    }
}
