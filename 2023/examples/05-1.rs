use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{Context, Result};

struct MapItem {
    dst: i64,
    src: i64,
    len: i64,
}

struct Map {
    dst: String,
    items: Vec<MapItem>,
}

impl Map {
    fn update(&self, item: &mut i64) {
        for MapItem { dst, src, len } in &self.items {
            if src <= item && *item < src + len {
                *item += dst - src;
                return;
            }
        }
    }
}

fn numbers(input: &str) -> Result<Vec<i64>> {
    input.split_whitespace().map(|x| Ok(x.parse()?)).collect()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut maps = Vec::<Map>::new();
    let mut lines = BufReader::new(input).lines();
    let seeds = lines.next().context("no seeds")??;
    let seeds = numbers(seeds.strip_prefix("seeds: ").context("no seeds:")?)?;
    assert!(lines.next().context("no maps")??.is_empty());
    let mut done = false;
    while !done {
        let dst = lines.next().context("no map")??;
        let (src, dst) = dst.split_once("-to-").context("no -to-")?;
        let dst = dst.strip_suffix(" map:").context("no map:")?.to_string();
        assert_eq!(src, maps.last().map_or("seed", |x| &x.dst));
        let mut items = Vec::new();
        loop {
            let line = match lines.next() {
                None => {
                    done = true;
                    break;
                }
                Some(x) => x?,
            };
            if line.is_empty() {
                break;
            }
            let item = numbers(&line)?;
            assert_eq!(item.len(), 3);
            items.push(MapItem { dst: item[0], src: item[1], len: item[2] });
        }
        maps.push(Map { dst, items });
    }
    assert_eq!(maps.last().unwrap().dst, "location");
    let mut items = seeds;
    for map in maps {
        for item in &mut items {
            map.update(item);
        }
    }
    writeln!(output, "{}", items.iter().min().unwrap())?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "84470622\n");
