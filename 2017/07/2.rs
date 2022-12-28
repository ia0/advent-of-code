use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct Value {
    weight: u64,
    children: Vec<String>,
}

#[derive(Debug)]
struct Tower {
    weight: u64,
    total: u64,
    normal: Option<u64>,
    next: Option<Box<Tower>>,
}

#[derive(Debug)]
struct Program {
    name: String,
    value: Value,
}

impl std::str::FromStr for Program {
    type Err = ();

    fn from_str(input: &str) -> Result<Program, ()> {
        let mut bytes = input.bytes();
        let mut name = String::new();
        for byte in &mut bytes {
            if b'a' <= byte && byte <= b'z' {
                name.push(byte as char);
            } else {
                assert_eq!(byte, b' ');
                break;
            }
        }
        assert_eq!(bytes.next(), Some(b'('));
        let mut weight = 0;
        for byte in &mut bytes {
            if b'0' <= byte && byte <= b'9' {
                weight *= 10;
                weight += (byte - b'0') as u64;
            } else {
                assert_eq!(byte, b')');
                break;
            }
        }
        match bytes.next() {
            None => {
                return Ok(Program {
                    name,
                    value: Value { weight, children: Vec::new() },
                })
            }
            Some(b' ') => (),
            Some(x) => panic!("unexpected {:#02x}", x),
        }
        assert_eq!(bytes.next(), Some(b'-'));
        assert_eq!(bytes.next(), Some(b'>'));
        assert_eq!(bytes.next(), Some(b' '));
        let mut children = Vec::new();
        loop {
            children.push(String::new());
            for byte in &mut bytes {
                if b'a' <= byte && byte <= b'z' {
                    children.last_mut().unwrap().push(byte as char);
                } else {
                    assert_eq!(byte, b',');
                    break;
                }
            }
            match bytes.next() {
                None => break,
                Some(b' ') => (),
                Some(x) => panic!("unexpected {:#02x}", x),
            }
        }
        Ok(Program { name, value: Value { weight, children } })
    }
}

fn find(
    todo: &HashMap<String, Value>,
    forest: &HashMap<String, Tower>,
) -> Option<String> {
    for (name, value) in todo.iter() {
        let mut ok = true;
        for child in value.children.as_slice() {
            if !forest.contains_key(child.as_str()) {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(name.to_owned());
        }
    }
    None
}

fn main() {
    let stdin = std::io::stdin();
    let mut todo = HashMap::new();
    for line in stdin.lock().lines() {
        let program: Program = line.unwrap().parse().unwrap();
        assert!(todo.insert(program.name, program.value).is_none());
    }
    let mut forest = HashMap::new();
    while let Some(name) = find(&todo, &forest) {
        let value = todo.remove(name.as_str()).unwrap();
        let weight = value.weight;
        let mut total = weight;
        let mut normal = None;
        let mut next = None;
        let mut weights = HashMap::new();
        for child in value.children.as_slice() {
            let child = forest.remove(child).unwrap();
            total += child.total;
            if child.next.is_some() {
                assert!(next.is_none());
                next = Some(Box::new(child));
            } else {
                weights.entry(child.total).or_insert(Vec::new()).push(child);
            }
        }
        if next.is_some() {
            assert!(weights.len() < 2);
        } else {
            assert!(weights.len() <= 2);
            for (weight, mut tower) in weights.drain() {
                if tower.len() == 1 {
                    assert!(next.is_none());
                    next = Some(Box::new(tower.pop().unwrap()));
                } else {
                    assert!(normal.is_none());
                    normal = Some(weight);
                }
            }
        }
        let tower = Tower { weight, total, normal, next };
        assert!(forest.insert(name, tower).is_none());
    }
    assert!(todo.is_empty());
    assert_eq!(forest.len(), 1);
    println!("{:#?}", forest);
}
