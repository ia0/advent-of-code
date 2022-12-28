use std::io::BufRead;

fn next(mut row: usize, mut col: usize, drow: bool, dinc: bool)
        -> (usize, usize) {
    if drow {
        if dinc { row += 1; } else { row -= 1; }
    } else {
        if dinc { col += 1; } else { col -= 1; }
    }
    (row, col)
}

fn main() {
    let stdin = std::io::stdin();
    let mut map = Vec::new();
    for line in stdin.lock().lines() {
        map.push(line.unwrap().into_bytes());
    }
    let mut row = 0;
    let mut col = map[row].iter().position(|x| *x == b'|').unwrap();
    let mut drow = true;
    let mut dinc = true;
    let mut count = 0;
    loop {
        count += 1;
        match map[row][col] {
            b'|' | b'-' => (),
            b'+' => {
                let (nrow, ncol) = next(row, col, drow, dinc);
                if map[nrow][ncol] == b' ' {
                    drow = !drow;
                    let (arow, acol) = next(row, col, drow, true);
                    let (brow, bcol) = next(row, col, drow, false);
                    assert!(map[arow][acol] == b' ' || map[brow][bcol] == b' ');
                    if map[arow][acol] != b' ' {
                        dinc = true;
                    } else if map[brow][bcol] != b' ' {
                        dinc = false;
                    } else {
                        break;
                    }
                }
            }
            b' ' => {
                count -= 1;
                break;
            }
            b'A' ... b'Z' => (),
            _ => {
                panic!("Unexpected map[{}][{}] = {:?}", row, col,
                       map[row][col] as char);
            }
        };
        let (nrow, ncol) = next(row, col, drow, dinc);
        row = nrow;
        col = ncol;
    }
    println!("{}", count);
}
