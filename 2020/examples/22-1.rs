use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct State {
    decks: [VecDeque<usize>; 2],
}

impl State {
    fn done(&self) -> bool {
        self.decks.iter().any(|x| x.is_empty())
    }

    fn step(&mut self) {
        let mut top: Vec<_> = (0 .. 2).map(|i| self.decks[i].pop_front().unwrap()).collect();
        let win = (top[1] > top[0]) as usize;
        if win == 1 {
            top.swap(0, 1);
        }
        for x in top {
            self.decks[win].push_back(x);
        }
    }

    fn result(&self) -> usize {
        let i = (0 .. 2).find(|&i| !self.decks[i].is_empty()).unwrap();
        self.decks[i].iter().rev().enumerate().map(|(x, y)| (x + 1) * y).sum()
    }
}

fn main() {
    let input = File::open("examples/22.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut state = State::default();
    for i in 0 .. 2 {
        assert_eq!(lines.next().unwrap().unwrap(), format!("Player {}:", i + 1));
        for line in &mut lines {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            state.decks[i].push_back(line.parse().unwrap());
        }
    }
    while !state.done() {
        state.step();
    }
    println!("{}", state.result());
}
