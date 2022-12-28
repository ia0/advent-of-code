use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug)]
enum Path {
    Vertical,
    Horizontal,
    RightTurn,
    LeftTurn,
    Intersection,
}
use Path::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Memory {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Cart {
    x: u32,
    y: u32,
    direction: Direction,
    memory: Memory,
}

impl Cart {
    fn step(&mut self, tracks: &[Vec<Option<Path>>]) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
        match tracks[self.y as usize][self.x as usize].as_ref().unwrap() {
            RightTurn => match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Down,
                Direction::Right => self.direction = Direction::Up,
            },
            LeftTurn => match self.direction {
                Direction::Up => self.direction = Direction::Left,
                Direction::Down => self.direction = Direction::Right,
                Direction::Left => self.direction = Direction::Up,
                Direction::Right => self.direction = Direction::Down,
            },
            Intersection => match self.memory {
                Memory::Left => {
                    match self.direction {
                        Direction::Up => self.direction = Direction::Left,
                        Direction::Down => self.direction = Direction::Right,
                        Direction::Left => self.direction = Direction::Down,
                        Direction::Right => self.direction = Direction::Up,
                    }
                    self.memory = Memory::Straight;
                }
                Memory::Straight => {
                    self.memory = Memory::Right;
                }
                Memory::Right => {
                    match self.direction {
                        Direction::Up => self.direction = Direction::Right,
                        Direction::Down => self.direction = Direction::Left,
                        Direction::Left => self.direction = Direction::Up,
                        Direction::Right => self.direction = Direction::Down,
                    }
                    self.memory = Memory::Left;
                }
            },
            _ => (),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut tracks = Vec::new();
    let mut carts = Vec::new();
    let mut occupied = HashSet::new();
    let mut y = 0;
    for line in stdin.lock().lines() {
        let mut row = Vec::new();
        let mut x = 0;
        for byte in line.unwrap().into_bytes() {
            match byte {
                b' ' => row.push(None),
                b'|' => row.push(Some(Vertical)),
                b'-' => row.push(Some(Horizontal)),
                b'/' => row.push(Some(RightTurn)),
                b'\\' => row.push(Some(LeftTurn)),
                b'+' => row.push(Some(Intersection)),
                b'^' => {
                    row.push(Some(Vertical));
                    carts.push(Cart { x, y, direction: Direction::Up, memory: Memory::Left });
                    assert!(occupied.insert((x, y)));
                }
                b'v' => {
                    row.push(Some(Vertical));
                    carts.push(Cart { x, y, direction: Direction::Down, memory: Memory::Left });
                    assert!(occupied.insert((x, y)));
                }
                b'<' => {
                    row.push(Some(Horizontal));
                    carts.push(Cart { x, y, direction: Direction::Left, memory: Memory::Left });
                    assert!(occupied.insert((x, y)));
                }
                b'>' => {
                    row.push(Some(Horizontal));
                    carts.push(Cart { x, y, direction: Direction::Right, memory: Memory::Left });
                    assert!(occupied.insert((x, y)));
                }
                _ => panic!(),
            }
            x += 1;
        }
        tracks.push(row);
        assert_eq!(tracks.last().unwrap().len(), tracks[0].len());
        y += 1;
    }
    loop {
        carts.sort_by_key(|cart| (cart.y, cart.x));
        for cart in carts.iter_mut() {
            assert!(occupied.remove(&(cart.x, cart.y)));
            cart.step(&tracks);
            if !occupied.insert((cart.x, cart.y)) {
                println!("{},{}", cart.x, cart.y);
                return;
            }
        }
    }
}
