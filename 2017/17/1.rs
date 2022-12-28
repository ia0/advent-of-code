#![feature(inclusive_range_syntax)]

#[derive(Debug)]
struct Node {
    value: usize,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct Buffer {
    head: Node,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer { head: Node { value: 0, next: None } }
    }
}

fn main() {
    let step = 386;
    let mut buffer = Buffer::new();
    let mut node = &mut buffer.head;
    for value in 1 ..= 2017 {
        for _ in 0 .. step {
            node = match node.next {
                None => &mut buffer.head,
                Some(ref mut next) => next,
            };
        }
        let next = node.next.take();
        node.next = Some(Box::new(Node { value, next }));
        node = match node.next {
            None => panic!(),
            Some(ref mut next) => next,
        };
    }
    println!("{}", node.next.as_mut().unwrap().value);
}
