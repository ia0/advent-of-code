use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Border(u16);

impl Border {
    fn new(x: impl Iterator<Item = bool>) -> Border {
        let mut r = 0;
        for x in x {
            r *= 2;
            r += x as u16;
        }
        Border(r)
    }

    fn flip(self) -> Border {
        Border(self.0.reverse_bits() >> 6)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Orientation {
    rotate: u8, // [0, 4)
    flip: bool,
}

#[derive(Debug)]
struct Tile {
    #[allow(dead_code)]
    content: Vec<Vec<bool>>,
    borders: [Border; 4],
}

impl Tile {
    fn new(content: Vec<Vec<bool>>) -> Tile {
        let borders = [
            Border::new(content[0].iter().cloned()),
            Border::new(content.iter().map(|x| x[9])),
            Border::new(content[9].iter().rev().cloned()),
            Border::new(content.iter().rev().map(|x| x[0])),
        ];
        Tile { content, borders }
    }
}

fn main() {
    let input = File::open("examples/20.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut tiles = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let id: usize =
            line.strip_prefix("Tile ").unwrap().strip_suffix(':').unwrap().parse().unwrap();
        let tile = Tile::new(
            (0 .. 10)
                .map(|_| {
                    lines
                        .next()
                        .unwrap()
                        .unwrap()
                        .bytes()
                        .map(|x| match x {
                            b'.' => false,
                            b'#' => true,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        );
        assert_eq!(lines.next().unwrap().unwrap(), "");
        assert!(tiles.insert(id, tile).is_none());
    }
    let mut borders: HashMap<Border, HashMap<usize, HashSet<Orientation>>> = HashMap::new();
    for (&id, tile) in &tiles {
        for rotate in 0 .. 4 {
            for &flip in &[false, true] {
                let mut border = tile.borders[rotate as usize];
                if flip {
                    border = border.flip();
                }
                assert!(borders
                    .entry(border)
                    .or_default()
                    .entry(id)
                    .or_default()
                    .insert(Orientation { rotate, flip }));
            }
        }
    }
    let corners: Vec<_> = tiles
        .iter()
        .filter(|(id, tile)| {
            let mut neighbors: Vec<HashSet<usize>> =
                tile.borders.iter().map(|b| borders[b].keys().cloned().collect()).collect();
            assert!(neighbors.iter().all(|x| x.contains(id)));
            neighbors.push(neighbors[0].clone());
            neighbors.windows(2).any(|w| w[0].len() == 1 && w[1].len() == 1)
        })
        .map(|(&id, _)| id)
        .collect();
    println!("{}", corners.iter().product::<usize>());
}
