use std::collections::HashMap;
use std::collections::VecDeque;

const PLAYERS: usize = 412;
const MARBLES: usize = 71646;

struct Marble {
    // front=current --right--> back
    data: VecDeque<usize>,
}

impl Marble {
    fn new() -> Marble {
        let mut data = VecDeque::new();
        data.push_front(0);
        Marble { data }
    }

    fn move_right(&mut self) {
        let value = self.data.pop_front().unwrap();
        self.data.push_back(value);
    }

    fn insert_right(&mut self, value: usize) {
        self.move_right();
        self.data.push_front(value);
    }

    fn move_left(&mut self) {
        let value = self.data.pop_back().unwrap();
        self.data.push_front(value);
    }

    fn remove_current(&mut self) -> usize {
        self.data.pop_front().unwrap()
    }
}

fn main() {
    let mut marble = Marble::new();
    let mut player = 1;
    let mut score = HashMap::new();
    for i in 1 ..= MARBLES {
        if i % 23 == 0 {
            let entry = score.entry(player).or_insert(0);
            *entry += i;
            for _ in 0 .. 7 {
                marble.move_left();
            }
            *entry += marble.remove_current();
        } else {
            marble.move_right();
            marble.insert_right(i);
        }
        player += 1;
        if player > PLAYERS {
            player = 1;
        }
    }
    println!("{}", score.values().max().unwrap());
}
