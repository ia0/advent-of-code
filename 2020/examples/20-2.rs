use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

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

impl Orientation {
    fn inverse(mut self) -> Orientation {
        self.rotate = (4 - self.rotate) % 4;
        self
    }
}

#[derive(Debug, Clone)]
struct Tile {
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

    fn apply(&self, orientation: Orientation) -> Tile {
        Tile::new(
            (0 .. 10)
                .map(|y| {
                    (0 .. 10)
                        .map(|mut x| {
                            if orientation.flip {
                                x = 9 - x;
                            }
                            match orientation.rotate {
                                0 => self.content[y][x],
                                1 => self.content[9 - x][y],
                                2 => self.content[9 - y][9 - x],
                                3 => self.content[x][9 - y],
                                _ => unreachable!(),
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

fn update(
    tiles: &HashMap<usize, Tile>, borders: &HashMap<Border, HashMap<usize, HashSet<Orientation>>>,
    x: usize, y: usize, used: &mut HashSet<usize>, result: &mut Vec<Vec<Tile>>,
) -> bool {
    let left = if x > 0 { Some(result[y][x - 1].borders[1].flip()) } else { None };
    let up = if y > 0 { Some(result[y - 1][x].borders[2].flip()) } else { None };
    let mut candidate = None;
    match (left, up) {
        (None, None) => unreachable!(),
        (Some(left), None) => {
            for (&id, orien) in &borders[&left] {
                if used.contains(&id) {
                    continue;
                }
                for orien in orien {
                    let tile = tiles[&id]
                        .apply(orien.inverse())
                        .apply(Orientation { rotate: 3, flip: false });
                    if borders[&tile.borders[0]].keys().all(|&k| k == id || used.contains(&k)) {
                        if candidate.is_some() {
                            return false;
                        }
                        candidate = Some((id, tile));
                    }
                }
            }
        }
        (None, Some(up)) => {
            for (&id, orien) in &borders[&up] {
                if used.contains(&id) {
                    continue;
                }
                for orien in orien {
                    let tile = tiles[&id].apply(orien.inverse());
                    if borders[&tile.borders[3]].keys().all(|&k| k == id || used.contains(&k)) {
                        if candidate.is_some() {
                            return false;
                        }
                        candidate = Some((id, tile));
                    }
                }
            }
        }
        (Some(left), Some(up)) => {
            for (&id, orien) in &borders[&up] {
                if used.contains(&id) {
                    continue;
                }
                for orien in orien {
                    let tile = tiles[&id].apply(orien.inverse());
                    if tile.borders[3] == left {
                        if candidate.is_some() {
                            return false;
                        }
                        candidate = Some((id, tile));
                    }
                }
            }
        }
    }
    match candidate {
        None => false,
        Some((id, tile)) => {
            assert!(used.insert(id));
            assert_eq!(result[y].len(), x);
            result[y].push(tile);
            true
        }
    }
}

fn solve(
    tiles: &HashMap<usize, Tile>, borders: &HashMap<Border, HashMap<usize, HashSet<Orientation>>>,
    first: usize,
) -> Option<Vec<Vec<Tile>>> {
    let mut result = vec![vec![tiles[&first].clone()]];
    let mut used = HashSet::new();
    used.insert(first);
    'main: while used.len() < tiles.len() {
        for y in 0 ..= result.len() {
            if y == result.len() {
                result.push(Vec::new());
            }
            let x = result[y].len();
            if y > 0 && x == result[0].len() {
                continue;
            }
            if update(&tiles, &borders, x, y, &mut used, &mut result) {
                continue 'main;
            }
        }
        return None;
    }
    Some(result)
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
    assert_eq!(corners.len(), 4);
    let image = corners.iter().find_map(|&x| solve(&tiles, &borders, x)).unwrap();
    let n = image.len();
    assert!(image.iter().all(|x| x.len() == n));
    let mut map = HashSet::new();
    for x in 0 .. 8 * n {
        for y in 0 .. 8 * n {
            let p = Coord { x: x as i64, y: y as i64 };
            if image[y / 8][x / 8].content[y % 8 + 1][x % 8 + 1] {
                assert!(map.insert(p));
            }
        }
    }
    let mut monster = HashSet::new();
    const PATTERN: &[Coord] = &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: -1 },
        Coord { x: 4, y: -1 },
        Coord { x: 5, y: 0 },
        Coord { x: 6, y: 0 },
        Coord { x: 7, y: -1 },
        Coord { x: 10, y: -1 },
        Coord { x: 11, y: 0 },
        Coord { x: 12, y: 0 },
        Coord { x: 13, y: -1 },
        Coord { x: 16, y: -1 },
        Coord { x: 17, y: 0 },
        Coord { x: 18, y: 0 },
        Coord { x: 18, y: 1 },
        Coord { x: 19, y: 0 },
    ];
    for &pos in &map {
        for &xy in &[false, true] {
            for &x in &[false, true] {
                for &y in &[false, true] {
                    let pattern: Vec<_> = PATTERN
                        .iter()
                        .cloned()
                        .map(|mut d| {
                            if xy {
                                std::mem::swap(&mut d.x, &mut d.y);
                            }
                            if x {
                                d.x = -d.x;
                            }
                            if y {
                                d.y = -d.y;
                            }
                            pos + d
                        })
                        .collect();
                    if pattern.iter().all(|p| map.contains(p)) {
                        for p in pattern {
                            monster.insert(p);
                        }
                    }
                }
            }
        }
    }
    println!("{}", map.len() - monster.len());
}
