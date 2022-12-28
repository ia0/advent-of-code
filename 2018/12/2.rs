use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufRead;

struct State {
    data: VecDeque<bool>,
    index: i32,
    rules: HashMap<Vec<bool>, bool>,
}

impl State {
    fn new(data: VecDeque<bool>, rules: HashMap<Vec<bool>, bool>) -> State {
        State { data, index: 0, rules }
    }

    fn fix_edges(&mut self) {
        let n = self.data.len();
        assert!(n >= 4);
        let mut left = 0;
        while left < 4 && !self.data.get(left).unwrap() {
            left += 1;
        }
        for _ in 0 .. 4 - left {
            self.data.push_front(false);
            self.index -= 1;
        }
        let mut right = 0;
        while right < 4 && !self.data.get(n - right - 1).unwrap() {
            right += 1;
        }
        for _ in 0 .. 4 - right {
            self.data.push_back(false);
        }
    }

    fn advance(&mut self) {
        self.fix_edges();
        let mut data = self.data.clone();
        for i in 0 .. self.data.len() - 4 {
            let rule: Vec<bool> = self.data.iter().skip(i).take(5).cloned().collect();
            *data.get_mut(i + 2).unwrap() = *self.rules.get(&rule).unwrap();
        }
        self.data = data;
    }

    fn answer(&self) -> i32 {
        let mut index = self.index;
        let mut answer = 0;
        for &pot in self.data.iter() {
            if pot {
                answer += index;
            }
            index += 1;
        }
        answer
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    assert_eq!(lines.next().unwrap().unwrap(), "");
    let mut rules = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let mut left = Vec::new();
        for c in line.chars().take(5) {
            left.push(c == '#');
        }
        assert!(rules.insert(left, line.chars().last().unwrap() == '#').is_none());
    }
    assert_eq!(rules.len(), 32);
    assert_eq!(rules.get(&[false; 5] as &[bool]), Some(&false));
    let mut state = VecDeque::new();
    for c in first_line.chars().skip(15) {
        state.push_back(c == '#');
    }
    let mut state = State::new(state, rules);
    for _ in 0 .. 200 {
        state.advance();
    }
    println!("{}", (state.answer() as u64) + 51 * (50000000000 - 200));
}
