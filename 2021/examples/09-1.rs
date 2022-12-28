use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

fn get(map: &[Vec<u8>], p: Coord) -> usize {
    (map[p.x as usize][p.y as usize] - b'0') as usize
}

fn main() {
    let input = File::open("examples/09.txt").unwrap();
    let map: Vec<_> = BufReader::new(input).lines().map(|x| x.unwrap().into_bytes()).collect();
    let frame = Coord { x: map.len() as i64, y: map[0].len() as i64 };
    assert!(map.iter().all(|x| x.len() == frame.y as usize));
    let mut sum = 0;
    for p in frame.iter() {
        if adventofcode::ADJACENT_PLUS
            .iter()
            .map(|&d| p + d)
            .filter(|&q| frame.contains(q))
            .all(|q| get(&map, p) < get(&map, q))
        {
            sum += 1 + get(&map, p);
        }
    }
    println!("{}", sum);
}
