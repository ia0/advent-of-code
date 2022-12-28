use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Read;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

struct Frame {
    input: HashSet<Coord>,
    output: HashSet<Coord>,
}

struct Doors {
    horizontal: HashSet<Coord>,
    vertical: HashSet<Coord>,
}

fn advance(pos: HashSet<Coord>, doors: &mut Doors, dir: u8) -> HashSet<Coord> {
    pos.into_iter()
        .map(|mut coord| match dir {
            b'N' => {
                doors.horizontal.insert(coord);
                coord.y -= 1;
                coord
            }
            b'S' => {
                coord.y += 1;
                doors.horizontal.insert(coord);
                coord
            }
            b'W' => {
                doors.vertical.insert(coord);
                coord.x -= 1;
                coord
            }
            b'E' => {
                coord.x += 1;
                doors.vertical.insert(coord);
                coord
            }
            _ => panic!(),
        })
        .collect()
}

fn main() {
    let stdin = std::io::stdin();
    let mut bytes = stdin.lock().bytes();
    let mut doors = Doors { horizontal: HashSet::new(), vertical: HashSet::new() };
    let mut stack = Vec::new();
    let mut pos = HashSet::new();
    pos.insert(Coord { x: 0, y: 0 });
    assert_eq!(bytes.next().unwrap().unwrap(), b'^');
    loop {
        let byte = bytes.next().unwrap().unwrap();
        match byte {
            b'$' => break,
            b'N' | b'E' | b'W' | b'S' => pos = advance(pos, &mut doors, byte),
            b'(' => stack.push(Frame { input: pos.clone(), output: HashSet::new() }),
            b'|' => {
                let frame = stack.last_mut().unwrap();
                for coord in pos {
                    frame.output.insert(coord);
                }
                pos = frame.input.clone();
            }
            b')' => pos = stack.pop().unwrap().output,
            _ => panic!(),
        }
    }
    assert_eq!(bytes.next().unwrap().unwrap(), b'\n');
    assert!(bytes.next().is_none());
    assert!(stack.is_empty());
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    visited.insert(Coord { x: 0, y: 0 }, 0);
    queue.push_back(Coord { x: 0, y: 0 });
    while let Some(coord) = queue.pop_front() {
        let dist = visited.get(&coord).unwrap() + 1;
        let mut push = |coord| {
            if let Entry::Vacant(entry) = visited.entry(coord) {
                entry.insert(dist);
                queue.push_back(coord);
            }
        };
        if doors.horizontal.contains(&coord) {
            push(Coord { x: coord.x, y: coord.y - 1 });
        }
        let south = Coord { x: coord.x, y: coord.y + 1 };
        if doors.horizontal.contains(&south) {
            push(south);
        }
        if doors.vertical.contains(&coord) {
            push(Coord { x: coord.x - 1, y: coord.y });
        }
        let east = Coord { x: coord.x + 1, y: coord.y };
        if doors.vertical.contains(&east) {
            push(east);
        }
    }
    println!("{}", visited.values().max().unwrap());
}
