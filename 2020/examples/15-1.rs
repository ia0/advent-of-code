fn solve(mut xs: Vec<usize>) {
    for _ in 0 .. 2020 - xs.len() {
        let last = xs.last().unwrap();
        let next = match xs[.. xs.len() - 1].iter().rposition(|x| x == last) {
            None => 0,
            Some(pos) => xs.len() - 1 - pos,
        };
        xs.push(next);
    }
    println!("{}", xs.last().unwrap());
}

fn main() {
    solve(vec![9, 3, 1, 0, 8, 4]);
}
