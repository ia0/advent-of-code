fn count(digits: usize, first: usize, needs_double: bool) -> usize {
    assert!(digits > 0);
    assert!(first < 10);
    if digits == 1 {
        if needs_double {
            return 0;
        } else {
            return 1;
        }
    }
    let mut result = 0;
    for second in first .. 10 {
        result += count(digits - 1, second, needs_double && first != second);
    }
    result
}

fn main() {
    // let min = b"156218";
    // let max = b"652527";
    println!(
        "{}",
        [
            (4, 6, true),
            (4, 7, true),
            (4, 8, true),
            (4, 9, true),
            (5, 6, true),
            (5, 7, true),
            (5, 8, true),
            (5, 9, true),
            (6, 2, true),
            (6, 3, true),
            (6, 4, true),
            (6, 5, true),
        ]
        .iter()
        .map(|&(digits, first, needs_double)| count(digits, first, needs_double))
        .sum::<usize>()
    );
}
