#[derive(Default)]
struct Link {
    prev: usize,
    next: usize,
}

struct Chain {
    head: usize,
    links: Vec<Link>,
}

impl Chain {
    fn new(input: &[u8; 9], length: usize) -> Chain {
        let head = (input[0] - b'1') as usize;
        let mut links = Vec::new();
        links.resize_with(length, Link::default);
        let mut chain = Chain { links, head };
        chain.link(length - 1, head);
        for i in 1 .. 9 {
            chain.link((input[i - 1] - b'1') as usize, (input[i] - b'1') as usize);
        }
        chain.link((input[8] - b'1') as usize, 9);
        for i in 10 .. length {
            chain.link(i - 1, i);
        }
        chain
    }

    fn link(&mut self, prev: usize, next: usize) {
        self.links[prev].next = next;
        self.links[next].prev = prev;
    }

    fn step(&mut self) {
        let n = self.links.len();
        let src = self.head;
        let span = self.read(src, 3);
        let mut dst = (src + n - 1) % n;
        while span.contains(&dst) {
            dst = (dst + n - 1) % n;
        }
        self.link(src, self.links[span[2]].next);
        self.link(span[2], self.links[dst].next);
        self.link(dst, span[0]);
        self.head = self.links[src].next;
    }

    fn read(&self, mut pos: usize, length: usize) -> Vec<usize> {
        let mut result = Vec::new();
        for _ in 0 .. length {
            pos = self.links[pos].next;
            result.push(pos);
        }
        result
    }
}

fn main() {
    let mut chain = Chain::new(b"589174263", 1_000_000);
    for _ in 0 .. 10_000_000 {
        chain.step();
    }
    println!("{}", chain.read(0, 2).iter().map(|x| x + 1).product::<usize>());
}
