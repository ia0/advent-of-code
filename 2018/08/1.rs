use std::io::BufRead;

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<i32>,
}

impl Tree {
    fn parse(tokens: &mut impl Iterator<Item = i32>) -> Tree {
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

    fn sum(&self) -> i32 {
        self.children.iter().map(|child| child.sum()).sum::<i32>()
            + self.metadata.iter().sum::<i32>()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut tokens = line.split_whitespace().map(|x| x.parse().unwrap());
    let root = Tree::parse(&mut tokens);
    assert!(tokens.next().is_none());
    println!("{}", root.sum());
}
