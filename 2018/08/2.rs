use std::io::BufRead;

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<usize>,
}

impl Tree {
    fn parse(tokens: &mut impl Iterator<Item = usize>) -> Tree {
        let children_len = tokens.next().unwrap();
        let metadata_len = tokens.next().unwrap();
        let mut children = Vec::new();
        for _ in 0 .. children_len {
            children.push(Tree::parse(tokens));
        }
        let mut metadata = Vec::new();
        for _ in 0 .. metadata_len {
            metadata.push(tokens.next().unwrap());
        }
        Tree { children, metadata }
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|&index| {
                    if 1 <= index && index <= self.children.len() {
                        self.children[index - 1].value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut tokens = line.split_whitespace().map(|x| x.parse().unwrap());
    let root = Tree::parse(&mut tokens);
    assert!(tokens.next().is_none());
    println!("{}", root.value());
}
