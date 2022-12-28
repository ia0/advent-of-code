use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Board {
    #[allow(dead_code)]
    board: Vec<Vec<usize>>,
    row_col: HashMap<usize, (usize, usize)>,
}

impl Board {
    fn new(board: Vec<Vec<usize>>) -> Board {
        let mut row_col = HashMap::new();
        for row in 0 .. 5 {
            for col in 0 .. 5 {
                assert!(row_col.insert(board[row][col], (row, col)).is_none());
            }
        }
        Board { board, row_col }
    }

    fn execute(mut self, xs: &[usize]) -> (usize, usize) {
        let mut rows = vec![0; 5];
        let mut cols = vec![0; 5];
        for i in 0 .. xs.len() {
            let (row, col) = match self.row_col.remove(&xs[i]) {
                None => continue,
                Some(x) => x,
            };
            rows[row] += 1;
            cols[col] += 1;
            if rows[row] == 5 || cols[col] == 5 {
                return (i, xs[i] * self.row_col.keys().sum::<usize>());
            }
        }
        unreachable!()
    }
}

fn main() {
    let input = File::open("examples/04.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let numbers: Vec<usize> =
        (lines.next().unwrap().unwrap()).split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards = Vec::new();
    loop {
        match lines.next() {
            None => break,
            Some(x) => assert_eq!(x.unwrap(), ""),
        }
        let mut board = Vec::new();
        for _ in 0 .. 5 {
            let row: Vec<usize> = (lines.next().unwrap().unwrap())
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            assert_eq!(row.len(), 5);
            board.push(row);
        }
        boards.push(Board::new(board));
    }
    println!("{}", boards.into_iter().map(|board| board.execute(&numbers)).min().unwrap().1);
}
