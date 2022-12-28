use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

struct Problem {
    map: Vec<Vec<u8>>,
    count: usize,
}

impl Problem {
    fn new(map: Vec<Vec<u8>>) -> Problem {
        Problem { map, count: 0 }
    }

    fn get(&mut self, p: Coord) -> &mut u8 {
        &mut self.map[p.y as usize][p.x as usize]
    }

    fn incr(&mut self, p: Coord) -> bool {
        let v = self.get(p);
        *v += 1;
        *v > 9
    }

    fn step(&mut self) {
        let frame = Coord { x: 10, y: 10 };
        let mut flash = HashSet::new();
        let mut todo: Vec<_> = frame.iter().collect();
        while let Some(p) = todo.pop() {
            if self.incr(p) && flash.insert(p) {
                for d in adventofcode::ADJACENT_STAR {
                    let q = p + d;
                    if frame.contains(q) {
                        todo.push(q);
                    }
                }
            }
        }
        for p in flash {
            *self.get(p) = 0;
            self.count += 1;
        }
    }
}

fn main() {
    let input = File::open("examples/11.txt").unwrap();
    let map: Vec<Vec<u8>> = BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().bytes().map(|x| x - b'0').collect())
        .collect();
    let mut problem = Problem::new(map);
    for _ in 0 .. 100 {
        problem.step();
    }
    println!("{}", problem.count);
}
