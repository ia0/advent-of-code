use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let num_cols = 25;
    let num_rows = 6;
    let file = File::open("examples/08.txt").unwrap();
    let input = BufReader::new(file).lines().next().unwrap().unwrap();
    let num_layers = input.len() / (num_rows * num_cols);
    assert_eq!(input.len(), num_layers * num_rows * num_cols);
    let mut image = vec![b'?'; num_rows * num_cols];
    for layer in input.as_bytes().chunks(num_rows * num_cols) {
        for (x, y) in image.iter_mut().zip(layer.iter()) {
            if *x != b'?' {
                continue;
            }
            match y {
                b'0' => *x = b' ',
                b'1' => *x = b'X',
                b'2' => (),
                _ => unreachable!(),
            }
        }
    }
    for row in image.chunks(num_cols) {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}
