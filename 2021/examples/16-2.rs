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
    #[allow(dead_code)]
    version: u8, // 3 bits
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

    fn eval(&self) -> usize {
        let packets = match &self.content {
            Content::Literal(x) => return *x,
            Content::Operator(x) => x,
        };
        let values: Vec<usize> = packets.iter().map(|p| p.eval()).collect();
        assert!(values.len() > 0);
        match self.type_id {
            0 => values.into_iter().sum(),
            1 => values.into_iter().product(),
            2 => values.into_iter().min().unwrap(),
            3 => values.into_iter().max().unwrap(),
            5 if values.len() == 2 => (values[0] > values[1]) as usize,
            6 if values.len() == 2 => (values[0] < values[1]) as usize,
            7 if values.len() == 2 => (values[0] == values[1]) as usize,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = File::open("examples/16.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let input = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());
    let input = HEXUPPER.decode(input.as_bytes()).unwrap();
    let mut bits = Bits::new(&input);
    println!("{}", Packet::parse(&mut bits).unwrap().eval());
    while let Some(bit) = bits.read(1) {
        assert_eq!(bit, 0);
    }
}
