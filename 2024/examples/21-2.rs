use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

#[derive(Debug, Copy, Clone)]
struct Choice {
    xy: usize,
    yx: usize,
}

impl Choice {
    fn eval(self, xy: bool, yx: bool) -> usize {
        match (xy, yx) {
            (true, true) => std::cmp::min(self.xy, self.yx),
            (true, false) => self.xy,
            (false, true) => self.yx,
            (false, false) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Cost {
    direct: HashMap<Coord, usize>,
    choice: HashMap<Coord, Choice>,
}

impl Cost {
    fn eval(&self, p: Coord, xy: bool, yx: bool) -> usize {
        let d = p.signum().len_plus();
        let e = match d {
            0 => 1,
            1 => self.direct[&p.signum()],
            2 => self.choice[&p.signum()].eval(xy, yx),
            _ => unreachable!(),
        };
        p.len_plus() + e - d
    }

    fn init() -> Cost {
        let direct = [Coord { x: -1, y: 0 }, Coord { x: 1, y: 0 }, Coord { x: 0, y: -1 }, Coord {
            x: 0,
            y: 1,
        }]
        .into_iter()
        .map(|p| (p, 2))
        .collect();
        let choice =
            [Coord { x: -1, y: -1 }, Coord { x: -1, y: 1 }, Coord { x: 1, y: -1 }, Coord {
                x: 1,
                y: 1,
            }]
            .into_iter()
            .map(|p| (p, Choice { xy: 3, yx: 3 }))
            .collect();
        Cost { direct, choice }
    }

    fn next(&self) -> Cost {
        let direct = HashMap::from([
            (
                Coord { x: -1, y: 0 },
                self.eval(Coord { x: -2, y: 1 }, false, true)
                    + self.eval(Coord { x: 2, y: -1 }, true, false),
            ),
            (
                Coord { x: 1, y: 0 },
                self.eval(Coord { x: 0, y: 1 }, true, true)
                    + self.eval(Coord { x: 0, y: -1 }, true, true),
            ),
            (
                Coord { x: 0, y: -1 },
                self.eval(Coord { x: -1, y: 0 }, true, true)
                    + self.eval(Coord { x: 1, y: 0 }, true, true),
            ),
            (
                Coord { x: 0, y: 1 },
                self.eval(Coord { x: -1, y: 1 }, true, true)
                    + self.eval(Coord { x: 1, y: -1 }, true, true),
            ),
        ]);
        let choice = HashMap::from([
            (Coord { x: -1, y: -1 }, Choice {
                xy: self.eval(Coord { x: -2, y: 1 }, false, true)
                    + self.eval(Coord { x: 1, y: -1 }, true, false)
                    + self.eval(Coord { x: 1, y: 0 }, true, true),
                yx: self.eval(Coord { x: -1, y: 0 }, true, true)
                    + self.eval(Coord { x: -1, y: 1 }, false, true)
                    + self.eval(Coord { x: 2, y: -1 }, true, false),
            }),
            (Coord { x: -1, y: 1 }, Choice {
                xy: self.eval(Coord { x: -2, y: 1 }, false, true)
                    + self.eval(Coord { x: 1, y: 0 }, true, true)
                    + self.eval(Coord { x: 1, y: -1 }, true, true),
                yx: self.eval(Coord { x: -1, y: 1 }, true, true)
                    + self.eval(Coord { x: -1, y: 0 }, true, true)
                    + self.eval(Coord { x: 2, y: -1 }, true, false),
            }),
            (Coord { x: 1, y: -1 }, Choice {
                xy: self.eval(Coord { x: 0, y: 1 }, true, true)
                    + self.eval(Coord { x: -1, y: -1 }, true, true)
                    + self.eval(Coord { x: 1, y: 0 }, true, true),
                yx: self.eval(Coord { x: -1, y: 0 }, true, true)
                    + self.eval(Coord { x: 1, y: 1 }, true, true)
                    + self.eval(Coord { x: 0, y: -1 }, true, true),
            }),
            (Coord { x: 1, y: 1 }, Choice {
                xy: self.eval(Coord { x: 0, y: 1 }, true, true)
                    + self.eval(Coord { x: -1, y: 0 }, true, true)
                    + self.eval(Coord { x: 1, y: -1 }, true, true),
                yx: self.eval(Coord { x: -1, y: 1 }, true, true)
                    + self.eval(Coord { x: 1, y: 0 }, true, true)
                    + self.eval(Coord { x: 0, y: -1 }, true, true),
            }),
        ]);
        Cost { direct, choice }
    }

    fn best(&self, code: &[u8], keys: &HashMap<u8, Coord>) -> usize {
        let good: HashSet<Coord> = keys.values().cloned().collect();
        let mut pos = keys[&b'A'];
        let mut total = 0;
        for x in code {
            let next = keys[x];
            let xy = good.contains(&Coord { x: next.x, y: pos.y });
            let yx = good.contains(&Coord { x: pos.x, y: next.y });
            total += self.eval(next - pos, xy, yx);
            pos = next;
        }
        total
    }
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
    let mut best = vec![Cost::init()];
    for _ in 0 .. 25 {
        best.push(best.last().unwrap().next());
    }
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let code = line?.into_bytes();
        assert_eq!(code.len(), 4);
        assert_eq!(code[3], b'A');
        let num: usize = std::str::from_utf8(&code[.. 3]).unwrap().parse()?;
        total += num * best.last().unwrap().best(&code, &numeric);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "293919502998014\n");
