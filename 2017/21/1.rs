#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]

use std::collections::HashMap;
use std::io::BufRead;

fn parse(input: &str) -> u32 {
    let mut r = 1;
    for c in input.chars() {
        match c {
            '.' => r = 2 * r,
            '#' => r = 2 * r + 1,
            '/' => (),
            _ => panic!(),
        }
    }
    r
}

struct Rotate {
    current: u32,
    iteration: i32,
}

fn rotate2(x: u32) -> u32 {
    let mut y = 1 << 4;
    y |= (x & 1 << 0) << 1;
    y |= (x & 1 << 1) << 2;
    y |= (x & 1 << 2) >> 2;
    y |= (x & 1 << 3) >> 1;
    y
}

fn rotate3(x: u32) -> u32 {
    let mut y = 1 << 9;
    y |= (x & 1 << 0) << 2;
    y |= (x & 1 << 1) << 4;
    y |= (x & 1 << 2) << 6;
    y |= (x & 1 << 3) >> 2;
    y |= x & 1 << 4;
    y |= (x & 1 << 5) << 2;
    y |= (x & 1 << 6) >> 6;
    y |= (x & 1 << 7) >> 4;
    y |= (x & 1 << 8) >> 2;
    y
}

fn rotate(x: u32) -> u32 {
    if x & 1 << 9 != 0 {
        rotate3(x)
    } else {
        assert!(x & 1 << 4 != 0);
        rotate2(x)
    }
}

fn flip2(x: u32) -> u32 {
    let mut y = 1 << 4;
    y |= (x & 1 << 0) << 1;
    y |= (x & 1 << 1) >> 1;
    y |= (x & 1 << 2) << 1;
    y |= (x & 1 << 3) >> 1;
    y
}

fn flip3(x: u32) -> u32 {
    let mut y = 1 << 9;
    y |= (x & 1 << 0) << 2;
    y |= x & 1 << 1;
    y |= (x & 1 << 2) >> 2;
    y |= (x & 1 << 3) << 2;
    y |= x & 1 << 4;
    y |= (x & 1 << 5) >> 2;
    y |= (x & 1 << 6) << 2;
    y |= x & 1 << 7;
    y |= (x & 1 << 8) >> 2;
    y
}

fn flip(x: u32) -> u32 {
    if x & 1 << 9 != 0 {
        flip3(x)
    } else {
        assert!(x & 1 << 4 != 0);
        flip2(x)
    }
}

impl Iterator for Rotate {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.iteration == 8 {
            return None;
        }
        let result = self.current;
        self.current = rotate(self.current);
        self.iteration += 1;
        if self.iteration == 4 {
            self.current = flip(self.current);
        }
        Some(result)
    }
}

fn cut<Get: Fn(u32) -> u32, Add: FnMut(u32)>(x: u32, get: Get, mut add: Add) {
    assert!(x & 1 << 16 != 0);
    let mut y: u64 = 0;
    for i in 0 .. 4 {
        let (sx, sy) = match i {
            0 => (0, 0),
            1 => (2, 3),
            2 => (8, 18),
            3 => (10, 21),
            _ => panic!(),
        };
        let mut z = 1 << 4;
        z |= (x & 1 << sx + 5) >> sx + 2;
        z |= (x & 1 << sx + 4) >> sx + 2;
        z |= (x & 1 << sx + 1) >> sx;
        z |= (x & 1 << sx) >> sx;
        let z = get(z);
        assert!(z & 1 << 9 != 0);
        for i in 0 .. 3 {
            for j in 0 .. 3 {
                if z & 1 << 3 * i + j != 0 {
                    y |= 1 << sy + 6 * i + j;
                }
            }
        }
    }
    for i in 0 .. 3 {
        for j in 0 .. 3 {
            let s = 12 * i + 2 * j;
            let mut z = 1 << 4;
            z |= (y & 1 << s + 7) >> s + 4;
            z |= (y & 1 << s + 6) >> s + 4;
            z |= (y & 1 << s + 1) >> s;
            z |= (y & 1 << s) >> s;
            add(z as u32);
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rules = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let rule: Vec<_> = line.split(" => ").collect();
        assert_eq!(rule.len(), 2);
        let right = parse(rule[1]);
        let lefts = Rotate {
            current: parse(rule[0]),
            iteration: 0,
        };
        for left in lefts {
            if let Some(x) = rules.insert(left, right) {
                assert_eq!(x, right);
            }
        }
    }
    let mut grid = HashMap::new();
    grid.insert(parse(".#./..#/###"), 1);
    for i in 0 .. 5 {
        let mut next = HashMap::new();
        for (left, count) in grid {
            let mut add = |x| *next.entry(x).or_insert(0) += count;
            match i % 3 {
                0 | 2 => add(rules[&left]),
                1 => cut(left, |x| rules[&x], add),
                _ => panic!(),
            }
        }
        grid = next;
    }
    let mut sum = 0;
    for (left, count) in grid {
        sum += unsafe { std::intrinsics::ctpop(left) - 1 } * count;
    }
    println!("{}", sum);
}
