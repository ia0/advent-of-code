use std::io::Read;

fn main() {
    let mut stack = Vec::new();
    let mut bytes = std::io::stdin().bytes();
    loop {
        let unit = bytes.next().unwrap().unwrap();
        if unit == b'\n' {
            break;
        }
        assert!(unit.is_ascii_alphabetic());
        if let Some(&prev) = stack.last() {
            if prev ^ unit == 0x20 {
                stack.pop();
                continue;
            }
        }
        stack.push(unit);
    }
    assert!(bytes.next().is_none());
    println!("{}", stack.len());
}
