use std::collections::HashMap;

fn solve(xs: &[usize]) {
    let mut pos = HashMap::new();
    let mut last = 0;
    for i in 0 .. 30000000 {
        let next = if i < xs.len() {
            xs[i]
        } else {
            match pos.get(&last) {
                None => 0,
                Some(pos) => i - 1 - pos,
            }
        };
        if i > 0 {
            pos.insert(last, i - 1);
        }
        last = next;
    }
    println!("{}", last);
}

fn main() {
    solve(&[9, 3, 1, 0, 8, 4]);
}
