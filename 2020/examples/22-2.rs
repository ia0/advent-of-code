use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Game {
    decks: [VecDeque<usize>; 2],
    seen: HashSet<Vec<usize>>,
}

impl Game {
    fn done(&self) -> bool {
        self.decks.iter().any(|x| x.is_empty())
    }

    fn step(&mut self) {
        if !self.seen.insert(self.fingerprint()) {
            self.decks[1].clear();
            return;
        }
        let mut top: Vec<_> = (0 .. 2).map(|i| self.decks[i].pop_front().unwrap()).collect();
        let win = if (0 .. 2).all(|i| self.decks[i].len() >= top[i]) {
            let mut game = Game::default();
            for i in 0 .. 2 {
                game.decks[i].extend(self.decks[i].iter().take(top[i]));
            }
            while !game.done() {
                game.step();
            }
            game.winner()
        } else {
            (top[1] > top[0]) as usize
        };
        if win == 1 {
            top.swap(0, 1);
        }
        self.decks[win].extend(top);
    }

    fn fingerprint(&self) -> Vec<usize> {
        let mut fingerprint: Vec<_> = self.decks[0].clone().into();
        fingerprint.push(0);
        fingerprint.extend(&self.decks[1]);
        fingerprint
    }

    fn winner(&self) -> usize {
        (0 .. 2).find(|&i| !self.decks[i].is_empty()).unwrap()
    }

    fn result(&self) -> usize {
        self.decks[self.winner()].iter().rev().enumerate().map(|(x, y)| (x + 1) * y).sum()
    }
}

fn main() {
    let input = File::open("examples/22.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut game = Game::default();
    for i in 0 .. 2 {
        assert_eq!(lines.next().unwrap().unwrap(), format!("Player {}:", i + 1));
        for line in &mut lines {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            game.decks[i].push_back(line.parse().unwrap());
        }
    }
    while !game.done() {
        game.step();
    }
    println!("{}", game.result());
}
