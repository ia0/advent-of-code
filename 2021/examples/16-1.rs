use std::fs::File;
use std::io::{BufRead, BufReader};

use data_encoding::HEXUPPER;

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8]) -> Bits<'a> {
        Bits { data, pos: 0 }
    }

    fn read(&mut self, n: usize) -> Option<usize> {
        let mut r = 0;
        for _ in 0 .. n {
            let p = self.pos;
            let x = *self.data.get(p / 8)? as usize;
            self.pos += 1;
            r = r << 1 | (x >> (7 - p % 8)) & 1;
        }
        Some(r)
    }
}

#[derive(Debug)]
struct Packet {
    version: u8, // 3 bits
    #[allow(dead_code)]
    type_id: u8, // 3 bits
    content: Content,
}

#[derive(Debug)]
enum Content {
    Literal(usize), // type_id 4
    Operator(Vec<Packet>),
}

enum Length {
    Bits(usize),
    Packets(usize),
}

impl Length {
    fn done(&self, pos: usize, len: usize) -> bool {
        match *self {
            Length::Bits(end) => pos >= end,
            Length::Packets(max) => len >= max,
        }
    }
}

impl Packet {
    fn parse(bits: &mut Bits) -> Option<Packet> {
        let version = bits.read(3)? as u8;
        let type_id = bits.read(3)? as u8;
        let content = if type_id == 4 {
            let mut literal = 0;
            loop {
                let chunk = bits.read(5)?;
                literal = literal << 4 | chunk & 15;
                if chunk & 1 << 4 == 0 {
                    break;
                }
            }
            Content::Literal(literal)
        } else {
            let mut packets = Vec::new();
            let length = if bits.read(1)? == 0 {
                let len = bits.read(15)?;
                Length::Bits(bits.pos + len)
            } else {
                Length::Packets(bits.read(11)?)
            };
            while !length.done(bits.pos, packets.len()) {
                packets.push(Packet::parse(bits)?);
            }
            Content::Operator(packets)
        };
        Some(Packet { version, type_id, content })
    }

    fn solve(&self) -> usize {
        let mut sum = self.version as usize;
        if let Content::Operator(packets) = &self.content {
            sum += packets.iter().map(|p| p.solve()).sum::<usize>();
        }
        sum
    }
}

fn main() {
    let input = File::open("examples/16.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let input = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());
    let input = HEXUPPER.decode(input.as_bytes()).unwrap();
    let mut bits = Bits::new(&input);
    println!("{}", Packet::parse(&mut bits).unwrap().solve());
    while let Some(bit) = bits.read(1) {
        assert_eq!(bit, 0);
    }
}
