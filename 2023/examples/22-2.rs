use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Range;

use adventofcode::order_pair_mut;
use anyhow::{ensure, Context, Result};

struct Brick {
    coord: [i64; 3],
    dir: usize,
    end: i64,
}

impl Brick {
    fn xys(&self) -> Box<dyn Iterator<Item = [i64; 2]>> {
        let Brick { coord, dir, end } = *self;
        let base = [coord[0], coord[1]];
        if dir == 2 {
            Box::new(std::iter::once(base))
        } else {
            Box::new((coord[dir] ..= end).map(move |x| {
                let mut r = base;
                r[dir] = x;
                r
            }))
        }
    }

    fn zs(&self) -> Range<i64> {
        if self.dir == 2 {
            self.coord[2] .. self.end + 1
        } else {
            self.coord[2] .. self.coord[2] + 1
        }
    }
}

fn parse_coord(input: &str) -> Result<[i64; 3]> {
    let coord = input.split(',').map(|x| Ok(x.parse()?)).collect::<Result<Vec<_>>>()?;
    coord.try_into().ok().context("bad coord")
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut bricks = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (a, b) = line.split_once('~').context("no ~")?;
        let mut coord = parse_coord(a)?;
        let end = parse_coord(b)?;
        let dir = (0 .. 3).filter(|&i| coord[i] != end[i]).collect::<Vec<_>>();
        ensure!(dir.len() <= 1);
        let dir = *dir.first().unwrap_or(&0);
        let mut end = end[dir];
        order_pair_mut(&mut coord[dir], &mut end);
        bricks.push(Brick { coord, dir, end });
    }
    let mut xys = HashMap::<[i64; 2], Vec<usize>>::new();
    for (i, brick) in bricks.iter().enumerate() {
        for xy in brick.xys() {
            xys.entry(xy).or_default().push(i);
        }
    }
    for bs in xys.values_mut() {
        bs.sort_by(|&j, &i| {
            let zi = bricks[i].zs();
            let zj = bricks[j].zs();
            if zi.end <= zj.start {
                Ordering::Less
            } else {
                assert!(zj.end <= zi.start);
                Ordering::Greater
            }
        });
    }
    let mut ground = HashMap::<[i64; 2], Vec<usize>>::new();
    let mut new_z = HashMap::<usize, i64>::new();
    while !xys.is_empty() {
        let next = xys
            .values()
            .map(|bs| *bs.last().unwrap())
            .min_by_key(|&i| bricks[i].zs().start)
            .unwrap();
        xys.retain(|_, bs| {
            if *bs.last().unwrap() == next {
                bs.pop();
            };
            !bs.is_empty()
        });
        let brick = &bricks[next];
        let z = brick
            .xys()
            .filter_map(|xy| {
                let i = *ground.get(&xy)?.last().unwrap();
                let zs = bricks[i].zs();
                Some(new_z[&i] + zs.end - zs.start)
            })
            .max()
            .unwrap_or(1);
        assert!(new_z.insert(next, z).is_none());
        for xy in brick.xys() {
            ground.entry(xy).or_default().push(next);
        }
    }
    for i in 0 .. bricks.len() {
        let new_z = new_z[&i];
        let old_z = std::mem::replace(&mut bricks[i].coord[2], new_z);
        if bricks[i].dir == 2 {
            bricks[i].end += new_z - old_z;
        }
    }
    let mut supports = (0 .. bricks.len()).map(|i| (i, HashSet::new())).collect::<HashMap<_, _>>();
    let mut supported_by = supports.clone();
    for bs in ground.values() {
        for w in bs.windows(2) {
            if bricks[w[0]].zs().end == bricks[w[1]].zs().start {
                supports.get_mut(&w[0]).unwrap().insert(w[1]);
                supported_by.get_mut(&w[1]).unwrap().insert(w[0]);
            }
        }
    }
    let mut total = 0;
    for i in 0 .. bricks.len() {
        let mut falls = HashSet::new();
        falls.insert(i);
        let mut todo = vec![i];
        while let Some(i) = todo.pop() {
            if supported_by[&i].iter().all(|j| falls.contains(j)) {
                falls.insert(i);
            }
            for &j in &supports[&i] {
                todo.push(j);
            }
        }
        total += falls.len() - 1;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "102770\n");
