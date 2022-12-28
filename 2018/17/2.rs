use std::collections::{hash_map, HashMap};
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Clay,
    FlowsTemporarily,
    FlowsIndefinitely,
    Settled,
}
use State::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    Top,
    Left,
    Right,
}
use Dir::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn down(self) -> Coord {
        Coord { x: self.x, y: self.y + 1 }
    }

    fn left(self) -> Coord {
        Coord { x: self.x - 1, y: self.y }
    }

    fn right(self) -> Coord {
        Coord { x: self.x + 1, y: self.y }
    }
}

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Frame {
    b: bool,
    x: Range,
    y: Range,
}

impl Frame {
    fn new() -> Frame {
        Frame { b: false, x: Range { min: 0, max: 0 }, y: Range { min: 0, max: 0 } }
    }

    fn update(&mut self, x: usize, y: usize) {
        if self.b {
            self.x.min = std::cmp::min(self.x.min, x);
            self.x.max = std::cmp::max(self.x.max, x);
            self.y.min = std::cmp::min(self.y.min, y);
            self.y.max = std::cmp::max(self.y.max, y);
        } else {
            self.b = true;
            self.x.min = x;
            self.x.max = x;
            self.y.min = y;
            self.y.max = y;
        }
    }
}

fn parse_numbers(input: &str) -> [usize; 3] {
    let input: Vec<usize> = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(input.len(), 3);
    let mut result = [0; 3];
    result.copy_from_slice(&input);
    result
}

// Returns whether settled for down and half-settled for left and right.
fn compute(ground: &mut HashMap<Coord, State>, frame: &mut Frame, source: Dir, pos: Coord) -> bool {
    if pos.y > frame.y.max {
        assert!(ground.insert(pos, FlowsIndefinitely).is_none());
        return false;
    }
    frame.x.min = std::cmp::min(frame.x.min, pos.x);
    frame.x.max = std::cmp::max(frame.x.max, pos.x);
    match ground.entry(pos) {
        hash_map::Entry::Occupied(entry) => {
            match entry.get() {
                Clay => return true,
                Settled => {
                    assert_eq!(source, Top);
                    return true;
                }
                FlowsIndefinitely => return false,
                _ => panic!(),
            };
        }
        hash_map::Entry::Vacant(entry) => entry.insert(FlowsTemporarily),
    };
    if !compute(ground, frame, Top, pos.down()) {
        assert_eq!(ground.insert(pos, FlowsIndefinitely), Some(FlowsTemporarily));
        return false;
    }
    let settled = match source {
        Top => {
            let left = compute(ground, frame, Right, pos.left());
            let right = compute(ground, frame, Left, pos.right());
            left && right
        }
        Left => compute(ground, frame, Left, pos.right()),
        Right => compute(ground, frame, Right, pos.left()),
    };
    if source == Top && settled {
        let mut left = pos.left();
        loop {
            let entry = ground.get_mut(&left).unwrap();
            match *entry {
                Clay => break,
                Settled | FlowsTemporarily => panic!(),
                FlowsIndefinitely => (),
            };
            *entry = Settled;
            left = left.left();
        }
        let mut right = pos.right();
        loop {
            let entry = ground.get_mut(&right).unwrap();
            match *entry {
                Clay => break,
                Settled | FlowsTemporarily => panic!(),
                FlowsIndefinitely => (),
            };
            *entry = Settled;
            right = right.right();
        }
        assert_eq!(ground.insert(pos, Settled), Some(FlowsTemporarily));
    } else {
        assert_eq!(ground.insert(pos, FlowsIndefinitely), Some(FlowsTemporarily));
    }
    settled
}

fn main() {
    let stdin = std::io::stdin();
    let mut frame = Frame::new();
    let mut ground = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let numbers = parse_numbers(&line);
        if line.as_bytes()[0] == b'x' {
            let x = numbers[0];
            for y in numbers[1] ..= numbers[2] {
                frame.update(x, y);
                ground.insert(Coord { x, y }, Clay);
            }
        } else {
            let y = numbers[0];
            for x in numbers[1] ..= numbers[2] {
                frame.update(x, y);
                ground.insert(Coord { x, y }, Clay);
            }
        }
    }
    compute(&mut ground, &mut frame, Top, Coord { x: 500, y: 0 });
    let mut count = 0;
    for x in frame.x.min ..= frame.x.max {
        for y in frame.y.min ..= frame.y.max {
            match ground.get(&Coord { x, y }) {
                Some(Settled) => count += 1,
                Some(FlowsTemporarily) => panic!(),
                _ => (),
            }
        }
    }
    println!("{}", count);
}
