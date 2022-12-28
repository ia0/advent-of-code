fn main() {
    let radius = std::env::args()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .unwrap_or(5);
    let input = std::env::args()
        .nth(2)
        .map(|x| x.parse().unwrap())
        .unwrap_or(368078);
    let size: i64 = 2 * radius + 1;
    let mut matrix = Vec::with_capacity(size as usize);
    for _ in 0 .. size {
        let mut row = Vec::with_capacity(size as usize);
        for _ in 0 .. size {
            row.push(0);
        }
        matrix.push(row);
    }
    let mut row = 0;
    let mut col = 0;
    matrix[radius as usize][radius as usize] = 1;
    while matrix[(radius + row) as usize][(radius + col) as usize] <= input {
        let rad = std::cmp::max(row.abs(), col.abs());
        if col == rad && row > -rad && row < rad {
            row += 1;
        } else if row == rad && col > -rad {
            col -= 1;
        } else if col == -rad && row > -rad {
            row -= 1;
        } else {
            assert!(row == -rad);
            assert!(-rad <= col && col <= rad);
            col += 1;
        }
        let r = (radius + row) as usize;
        let c = (radius + col) as usize;
        matrix[r][c] += matrix[r - 1][c - 1];
        matrix[r][c] += matrix[r - 1][c];
        matrix[r][c] += matrix[r - 1][c + 1];
        matrix[r][c] += matrix[r][c - 1];
        matrix[r][c] += matrix[r][c + 1];
        matrix[r][c] += matrix[r + 1][c - 1];
        matrix[r][c] += matrix[r + 1][c];
        matrix[r][c] += matrix[r + 1][c + 1];
    }
    println!("{}", matrix[(radius + row) as usize][(radius + col) as usize]);
}
