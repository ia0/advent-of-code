use std::io::Read;

fn length(polymer: &[u8], ignore: u8) -> usize {
    let mut stack = Vec::new();
    for unit in polymer {
        assert!(unit.is_ascii_alphabetic());
        if unit & !0x20 == ignore {
            continue;
        }
        if let Some(&prev) = stack.last() {
            if prev ^ unit == 0x20 {
                stack.pop();
                continue;
            }
        }
        stack.push(unit);
    }
    stack.len()
}

fn main() {
    let mut bytes: Vec<u8> = std::io::stdin().bytes().map(|x| x.unwrap()).collect();
    assert_eq!(bytes.last(), Some(&b'\n'));
    bytes.pop();
    println!("{}", (b'A' ..= b'Z').map(|x| length(&bytes, x)).min().unwrap());
}
