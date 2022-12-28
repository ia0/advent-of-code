use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
enum Event {
    Shift(usize),
    Sleep,
    WakeUp,
}

fn parse(line: &str) -> Event {
    let token = line.split_whitespace().nth(3).unwrap();
    if token == "asleep" {
        Event::Sleep
    } else if token == "up" {
        Event::WakeUp
    } else {
        let (head, tail) = token.split_at(1);
        assert_eq!(head, "#");
        Event::Shift(tail.parse().unwrap())
    }
}

fn minute(line: &str) -> usize {
    line.split(|x| [':', ']'].contains(&x)).nth(1).unwrap().parse().unwrap()
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    lines.sort();

    let mut sleep = HashMap::new();
    let mut last_id = None;
    let mut sleeping = None;
    for line in lines {
        match parse(&line) {
            Event::Shift(id) => {
                assert!(sleeping.is_none());
                last_id = Some(id);
            }
            Event::Sleep => {
                assert!(sleeping.is_none());
                sleeping = Some(minute(&line));
            }
            Event::WakeUp => {
                assert!(sleeping.is_some());
                assert!(last_id.is_some());
                let entry = sleep.entry(last_id.unwrap()).or_insert(vec![0; 60]);
                for i in sleeping.unwrap() .. minute(&line) {
                    entry[i] += 1;
                }
                sleeping = None;
            }
        }
    }

    let mut guard = 0;
    let mut minute = 0;
    let mut best = 0;
    for (&id, minutes) in sleep.iter() {
        for (i, &x) in minutes.iter().enumerate() {
            if x > best {
                best = x;
                minute = i;
                guard = id;
            }
        }
    }

    println!("{}", guard * minute);
}
