fn main() {
    let digits = std::fs::read_to_string("examples/16.txt").unwrap();
    let offset: usize = digits[.. 7].parse().unwrap();
    let mut digits = digits.into_bytes();
    assert_eq!(digits.last(), Some(&b'\n'));
    digits.pop();
    for digit in &mut digits {
        assert!(b'0' <= *digit && *digit <= b'9');
        *digit -= b'0';
    }
    let mut tail: Vec<u8> =
        digits.iter().rev().cycle().take(digits.len() * 10000 - offset).cloned().collect();
    for _ in 0 .. 100 {
        tail = tail
            .into_iter()
            .scan(0, |acc, x| {
                *acc += x;
                *acc %= 10;
                Some(*acc)
            })
            .collect();
    }
    tail.reverse();
    for i in 0 .. 8 {
        print!("{}", (b'0' + tail[i]) as char);
    }
    println!();
}
