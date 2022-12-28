#![feature(inclusive_range_syntax)]

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut programs: Vec<u8> = (b'a' ..= b'p').collect();
    let len = programs.len();
    let mut first: usize = 0;
    for command in line.split(',') {
        let command = command.as_bytes();
        match command[0] {
            b's' => {
                let n: usize = std::str::from_utf8(&command[1 ..])
                    .unwrap()
                    .parse()
                    .unwrap();
                assert!(n <= len);
                first += len - n;
            }
            b'x' => {
                let ab: Vec<_> = command[1 ..].split(|x| *x == b'/').collect();
                assert_eq!(ab.len(), 2);
                let a: usize =
                    std::str::from_utf8(ab[0]).unwrap().parse().unwrap();
                let b: usize =
                    std::str::from_utf8(ab[1]).unwrap().parse().unwrap();
                programs.swap((first + a) % len, (first + b) % len);
            }
            b'p' => {
                assert_eq!(command.len(), 4);
                assert_eq!(command[2], b'/');
                let a = programs.iter().position(|x| *x == command[1]).unwrap();
                let b = programs.iter().position(|x| *x == command[3]).unwrap();
                programs.swap(a, b);
            }
            _ => panic!(),
        };
    }
    first %= len;
    println!(
        "{}{}",
        std::str::from_utf8(&programs[first ..]).unwrap(),
        std::str::from_utf8(&programs[.. first]).unwrap()
    );
}
