use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct Value {
    weight: u64,
    children: Vec<String>,
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
    forest: &HashMap<String, Value>,
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
    while let Some(next) = find(&todo, &forest) {
        let value = todo.remove(next.as_str()).unwrap();
        for child in value.children.as_slice() {
            assert!(forest.remove(child).is_some());
        }
        assert!(forest.insert(next, value).is_none());
    }
    println!("todo = {:?}", todo);
    println!("forest = {:?}", forest);
}
