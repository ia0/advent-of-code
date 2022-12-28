use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

struct Image {
    pixels: HashSet<Coord>,
    color: u8,
}

fn decode(algorithm: &[u8; 512], pattern: [u8; 9]) -> u8 {
    let mut pos = 0;
    for i in 0 .. 9 {
        pos = 2 * pos + (pattern[i] == b'#') as usize;
    }
    algorithm[pos]
}

fn opposite(x: u8) -> u8 {
    match x {
        b'#' => b'.',
        b'.' => b'#',
        _ => unreachable!(),
    }
}

impl Image {
    fn step(&mut self, algorithm: &[u8; 512]) {
        let mut todo = HashSet::new();
        for &p in &self.pixels {
            todo.insert(p);
            for d in adventofcode::ADJACENT_STAR {
                todo.insert(p + d);
            }
        }
        let new_color = opposite(decode(algorithm, [opposite(self.color); 9]));
        let mut new_pixels = HashSet::new();
        for p in todo {
            if decode(algorithm, self.read(p)) == new_color {
                assert!(new_pixels.insert(p));
            }
        }
        self.pixels = new_pixels;
        self.color = new_color;
    }

    fn read(&self, p: Coord) -> [u8; 9] {
        let mut r = Vec::new();
        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                let d = Coord { x, y };
                if self.pixels.contains(&(p + d)) {
                    r.push(self.color);
                } else {
                    r.push(opposite(self.color));
                }
            }
        }
        r.try_into().unwrap()
    }
}

fn main() {
    let input = File::open("examples/20.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let algorithm: [u8; 512] = lines.next().unwrap().unwrap().into_bytes().try_into().unwrap();
    assert!(lines.next().unwrap().unwrap().is_empty());
    let mut pixels = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.unwrap().bytes().enumerate() {
            match c {
                b'.' => (),
                b'#' => assert!(pixels.insert(Coord { x: x as i64, y: y as i64 })),
                _ => unreachable!(),
            }
        }
    }
    let mut image = Image { pixels, color: b'#' };
    for _ in 0 .. 2 {
        image.step(&algorithm);
    }
    println!("{}", image.pixels.len());
}
