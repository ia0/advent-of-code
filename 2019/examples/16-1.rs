fn main() {
    let mut digits = std::fs::read_to_string("examples/16.txt").unwrap().into_bytes();
    assert_eq!(digits.last(), Some(&b'\n'));
    digits.pop();
    for digit in &mut digits {
        assert!(b'0' <= *digit && *digit <= b'9');
        *digit -= b'0';
    }
    for _ in 0 .. 100 {
        let mut next_digits = Vec::new();
        for i in 0 .. digits.len() {
            next_digits.push(
                (digits
                    .iter()
                    .zip(
                        [0i64, 1, 0, -1]
                            .iter()
                            .map(|&x| std::iter::repeat(x).take(i + 1))
                            .flatten()
                            .cycle()
                            .skip(1),
                    )
                    .map(|(&x, y)| (x as i64) * y)
                    .sum::<i64>()
                    .abs()
                    % 10) as u8,
            );
        }
        digits = next_digits;
    }
    for i in 0 .. 8 {
        print!("{}", (b'0' + digits[i]) as char);
    }
    println!();
}
