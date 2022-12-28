use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let num_cols = 25;
    let num_rows = 6;
    let file = File::open("examples/08.txt").unwrap();
    let input = BufReader::new(file).lines().next().unwrap().unwrap();
    let image = input.as_bytes();
    let num_layers = image.len() / (num_rows * num_cols);
    assert_eq!(image.len(), num_layers * num_rows * num_cols);
    let (_, x, y) = image
        .chunks(num_rows * num_cols)
        .map(|layer| {
            layer.iter().fold((0, 0, 0), |(x, y, z), w| match w {
                b'0' => (x + 1, y, z),
                b'1' => (x, y + 1, z),
                b'2' => (x, y, z + 1),
                _ => unreachable!(),
            })
        })
        .min()
        .unwrap();
    println!("{}", x * y);
}
