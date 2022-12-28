#![feature(inclusive_range_syntax)]

use std::io::BufRead;

struct Permutation(Vec<u8>);

impl Permutation {
    fn new() -> Permutation {
        Permutation((0 .. 16).collect())
    }

    fn trans(x: usize, y: usize) -> Permutation {
        let mut r = Permutation::new();
        r.0[x] = y as u8;
        r.0[y] = x as u8;
        r
    }

    fn cycle(x: usize) -> Permutation {
        let mut r = Vec::new();
        let l = 16 - x;
        for i in l .. 16 {
            r.push(i as u8);
        }
        for i in 0 .. l {
            r.push(i as u8);
        }
        Permutation(r)
    }

    fn mult(xs: &[u8], ys: &[u8]) -> Permutation {
        let mut r = Vec::new();
        for x in xs {
            r.push(ys[*x as usize])
        }
        Permutation(r)
    }
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut front = Permutation::new();
    let mut back = Permutation::new();
    for command in line.split(',') {
        let command = command.as_bytes();
        match command[0] {
            b's' => {
                let t = Permutation::cycle(
                    std::str::from_utf8(&command[1 ..])
                        .unwrap()
                        .parse()
                        .unwrap(),
                );
                front = Permutation::mult(&t.0, &front.0);
            }
            b'x' => {
                let ab: Vec<_> = command[1 ..].split(|x| *x == b'/').collect();
                assert_eq!(ab.len(), 2);
                let a: usize =
                    std::str::from_utf8(ab[0]).unwrap().parse().unwrap();
                let b: usize =
                    std::str::from_utf8(ab[1]).unwrap().parse().unwrap();
                let t = Permutation::trans(a, b);
                front = Permutation::mult(&t.0, &front.0);
            }
            b'p' => {
                let t = Permutation::trans(
                    (command[1] - b'a') as usize,
                    (command[3] - b'a') as usize,
                );
                back = Permutation::mult(&back.0, &t.0);
            }
            _ => panic!(),
        };
    }
    let mut n = 1000 * 1000 * 1000;
    let mut front_result = Permutation::new();
    let mut back_result = Permutation::new();
    while n > 0 {
        if n % 2 == 1 {
            front_result = Permutation::mult(&front_result.0, &front.0);
            back_result = Permutation::mult(&back_result.0, &back.0);
        }
        front = Permutation::mult(&front.0, &front.0);
        back = Permutation::mult(&back.0, &back.0);
        n /= 2;
    }
    front_result = Permutation::mult(&front_result.0, &back_result.0);
    for x in front_result.0.iter() {
        print!("{}", (b'a' + x) as char);
    }
    println!();
}
