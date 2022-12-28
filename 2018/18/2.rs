use std::collections::HashMap;
use std::io::BufRead;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}
use Acre::*;

fn count(acres: &[Acre]) -> (usize, usize) {
    let mut num_trees = 0;
    let mut num_lumberyard = 0;
    for acre in acres {
        match acre {
            OpenGround => (),
            Trees => num_trees += 1,
            Lumberyard => num_lumberyard += 1,
        }
    }
    (num_trees, num_lumberyard)
}

struct Area {
    width: usize,
    data: Vec<Acre>,
}

impl Area {
    fn new() -> Area {
        Area { width: 0, data: Vec::new() }
    }

    fn add(&mut self, line: Vec<u8>) {
        if self.width == 0 {
            self.width = line.len();
        }
        assert_eq!(line.len(), self.width);
        for acre in line {
            self.data.push(match acre {
                b'.' => OpenGround,
                b'|' => Trees,
                b'#' => Lumberyard,
                _ => panic!(),
            });
        }
    }

    fn get(&self, x: usize, y: usize) -> Acre {
        self.data[y * self.width + x]
    }

    fn adjacent_acres(&self, x: usize, y: usize) -> Vec<Acre> {
        let mut r = Vec::new();
        if x > 0 {
            if y > 0 {
                r.push(self.get(x - 1, y - 1));
            }
            r.push(self.get(x - 1, y));
            if y < self.width - 1 {
                r.push(self.get(x - 1, y + 1));
            }
        }
        if y > 0 {
            r.push(self.get(x, y - 1));
        }
        if y < self.width - 1 {
            r.push(self.get(x, y + 1));
        }
        if x < self.width - 1 {
            if y > 0 {
                r.push(self.get(x + 1, y - 1));
            }
            r.push(self.get(x + 1, y));
            if y < self.width - 1 {
                r.push(self.get(x + 1, y + 1));
            }
        }
        r
    }

    fn count_adjacent(&self, x: usize, y: usize) -> (usize, usize) {
        count(&self.adjacent_acres(x, y))
    }

    fn advance(&mut self) -> Vec<Acre> {
        let mut next_data = Vec::new();
        assert_eq!(self.data.len() % self.width, 0);
        for y in 0 .. self.data.len() / self.width {
            for x in 0 .. self.width {
                let (num_trees, num_lumberyard) = self.count_adjacent(x, y);
                next_data.push(match self.get(x, y) {
                    OpenGround if num_trees >= 3 => Trees,
                    Trees if num_lumberyard >= 3 => Lumberyard,
                    Lumberyard if num_trees * num_lumberyard == 0 => OpenGround,
                    x => x,
                });
            }
        }
        std::mem::swap(&mut self.data, &mut next_data);
        next_data
    }

    fn resource_value(&self) -> usize {
        let (num_trees, num_lumberyard) = count(&self.data);
        num_trees * num_lumberyard
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut area = Area::new();
    for line in stdin.lock().lines() {
        area.add(line.unwrap().into_bytes());
    }
    const MINUTES: usize = 1000000000;
    let mut history = HashMap::new();
    for i in 0 .. MINUTES {
        if let Some(j) = history.insert(area.advance(), i) {
            let delta = i - j;
            for _ in 0 .. (MINUTES - i - 1) % delta {
                area.advance();
            }
            println!("{}", area.resource_value());
            break;
        }
    }
}
