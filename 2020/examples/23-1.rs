fn main() {
    let mut cups = *b"589174263";
    for _ in 0 .. 100 {
        let d = (4 .. 9).max_by_key(|&i| (9 + cups[i] - cups[0]) % 9).unwrap();
        cups[1 ..= d].rotate_left(3);
        cups.rotate_left(1);
    }
    let first = cups.iter().position(|&x| x == b'1').unwrap();
    cups.rotate_left(first);
    println!("{}", std::str::from_utf8(&cups[1 ..]).unwrap());
}
