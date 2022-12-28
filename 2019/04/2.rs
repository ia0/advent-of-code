#![feature(is_sorted)]

fn has_double(x: Vec<u8>) -> bool {
    let mut count = [0; 10];
    for i in x {
        count[(i - b'0') as usize] += 1;
    }
    count.iter().any(|&x| x == 2)
}

fn is_password(x: usize) -> bool {
    let x = format!("{}", x).into_bytes();
    debug_assert_eq!(x.len(), 6);
    debug_assert!(x.iter().all(|&x| b'0' <= x && x <= b'9'));
    x.is_sorted() && has_double(x)
}

fn main() {
    let mut count = 0;
    for x in 156218 ..= 652527 {
        if is_password(x) {
            count += 1;
        }
    }
    println!("{}", count);
}
