use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Map(u32);

impl Map {
    fn add(self, x: i8, y: i8) -> Map {
        assert!(x != 2 || y != 2);
        Map(self.0 | 1 << 5 * y + x)
    }

    fn remove(self, x: i8, y: i8) -> Map {
        assert!(x != 2 || y != 2);
        Map(self.0 & !(1 << 5 * y + x))
    }

    fn bug(self, x: i8, y: i8) -> bool {
        0 <= x && x < 5 && 0 <= y && y < 5 && (self.0 & 1 << 5 * y + x != 0)
    }

    fn bugs(self, outer: Map, inner: Map, x: i8, y: i8) -> usize {
        let mut count = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .filter(|&&(x, y)| self.bug(x, y))
            .count();
        if x == 0 {
            count += outer.bug(1, 2) as usize;
        }
        if x == 4 {
            count += outer.bug(3, 2) as usize;
        }
        if y == 0 {
            count += outer.bug(2, 1) as usize;
        }
        if y == 4 {
            count += outer.bug(2, 3) as usize;
        }
        if x == 2 && y == 1 {
            count += (0 .. 5).filter(|&x| inner.bug(x, 0)).count();
        }
        if x == 2 && y == 3 {
            count += (0 .. 5).filter(|&x| inner.bug(x, 4)).count();
        }
        if x == 1 && y == 2 {
            count += (0 .. 5).filter(|&y| inner.bug(0, y)).count();
        }
        if x == 3 && y == 2 {
            count += (0 .. 5).filter(|&y| inner.bug(4, y)).count();
        }
        count
    }

    fn step(self, outer: Map, inner: Map) -> Map {
        let mut r = self;
        for y in 0 .. 5 {
            for x in 0 .. 5 {
                if x == 2 && y == 2 {
                    assert!(!self.bug(x, y));
                    continue;
                }
                let bugs = self.bugs(outer, inner, x, y);
                if self.bug(x, y) {
                    if bugs != 1 {
                        r = r.remove(x, y);
                    }
                } else {
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
    let n = 200;
    let mut maps = vec![Map::default(); n + 3];
    maps[n / 2 + 1] = map;
    for _ in 0 .. n {
        maps = (0 .. maps.len())
            .map(|i| {
                maps[i].step(
                    if i > 0 { maps[i - 1] } else { Map::default() },
                    if i < maps.len() - 1 { maps[i + 1] } else { Map::default() },
                )
            })
            .collect();
    }
    println!("{}", maps.iter().map(|map| map.0.count_ones()).sum::<u32>());
}
