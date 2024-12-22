use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn push(code: &mut Vec<u8>, mut dist: i64, neg: u8, pos: u8) {
    while dist != 0 {
        if dist < 0 {
            code.push(neg);
            dist += 1;
        } else {
            code.push(pos);
            dist -= 1;
        }
    }
}

fn find(code: &[u8], keys: &HashMap<u8, Coord>) -> Vec<u8> {
    let good: HashSet<Coord> = keys.values().cloned().collect();
    let mut res = Vec::new();
    let mut pos = keys[&b'A'];
    for x in code {
        let next = keys[x];
        if !good.contains(&Coord { x: next.x, y: pos.y }) || (pos.y < next.y && pos.x < next.x) {
            assert!(good.contains(&Coord { x: pos.x, y: next.y }));
            push(&mut res, next.y - pos.y, b'^', b'v');
            push(&mut res, next.x - pos.x, b'<', b'>');
        } else {
            assert!(good.contains(&Coord { x: next.x, y: pos.y }));
            push(&mut res, next.x - pos.x, b'<', b'>');
            push(&mut res, next.y - pos.y, b'^', b'v');
        }
        res.push(b'A');
        pos = next;
    }
    res
}

#[allow(dead_code)]
fn brute(code: &[u8], keys: &HashMap<u8, Coord>) -> Vec<Vec<u8>> {
    let good: HashSet<Coord> = keys.values().cloned().collect();
    let mut result = Vec::new();
    let mut br = Vec::new();
    loop {
        let mut pos = keys[&b'A'];
        let mut res = Vec::new();
        let mut i = 0;
        for x in code {
            let next = keys[x];
            let xy = good.contains(&Coord { x: next.x, y: pos.y });
            let yx = good.contains(&Coord { x: pos.x, y: next.y });
            assert!(xy | yx);
            let choice = if xy && yx && pos.x != next.x && pos.y != next.y {
                if result.is_empty() {
                    br.push(false);
                }
                let b = br[i];
                i += 1;
                b
            } else {
                yx
            };
            if choice {
                push(&mut res, next.y - pos.y, b'^', b'v');
                push(&mut res, next.x - pos.x, b'<', b'>');
            } else {
                push(&mut res, next.x - pos.x, b'<', b'>');
                push(&mut res, next.y - pos.y, b'^', b'v');
            }
            res.push(b'A');
            pos = next;
        }
        result.push(res);
        assert_eq!(br.len(), i);
        for b in &mut br {
            *b = !*b;
            if *b {
                break;
            }
        }
        if br.iter().all(|x| !x) {
            break;
        }
    }
    result
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let numeric: HashMap<u8, Coord> = [
        (b'7', Coord { x: 0, y: 0 }),
        (b'8', Coord { x: 1, y: 0 }),
        (b'9', Coord { x: 2, y: 0 }),
        (b'4', Coord { x: 0, y: 1 }),
        (b'5', Coord { x: 1, y: 1 }),
        (b'6', Coord { x: 2, y: 1 }),
        (b'1', Coord { x: 0, y: 2 }),
        (b'2', Coord { x: 1, y: 2 }),
        (b'3', Coord { x: 2, y: 2 }),
        (b'0', Coord { x: 1, y: 3 }),
        (b'A', Coord { x: 2, y: 3 }),
    ]
    .into_iter()
    .collect();
    let directional: HashMap<u8, Coord> = [
        (b'^', Coord { x: 1, y: 0 }),
        (b'A', Coord { x: 2, y: 0 }),
        (b'<', Coord { x: 0, y: 1 }),
        (b'v', Coord { x: 1, y: 1 }),
        (b'>', Coord { x: 2, y: 1 }),
    ]
    .into_iter()
    .collect();
    #[cfg(any())]
    for code in [b"<^", b"^<", b">^", b"^>", b"<v", b"v<", b">v", b"v>"] {
        let mut best = usize::MAX;
        for code in brute(code, &directional) {
            for code in brute(&code, &directional) {
                best = std::cmp::min(best, code.len());
            }
        }
        eprintln!("{} {best}", std::str::from_utf8(code).unwrap());
    }
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let mut code = line?.into_bytes();
        assert_eq!(code.len(), 4);
        assert_eq!(code[3], b'A');
        let num: usize = std::str::from_utf8(&code[.. 3]).unwrap().parse()?;
        for i in 0 .. 3 {
            code = find(&code, if i == 0 { &numeric } else { &directional });
        }
        total += num * code.len();
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "238078\n");
