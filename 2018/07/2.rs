use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct Work {
    step: u8,
    time: usize,
}

impl Work {
    fn new(step: u8) -> Work {
        Work { step, time: (61 + step - b'A') as usize }
    }
}

struct Problem {
    dependencies: HashMap<u8, HashSet<u8>>,
    workers: Vec<Option<Work>>,
    ready: BTreeSet<u8>,
    time: usize,
}

impl Problem {
    fn new(dependencies: HashMap<u8, HashSet<u8>>) -> Problem {
        let mut ready = BTreeSet::new();
        for (&step, dependencies) in dependencies.iter() {
            if dependencies.is_empty() {
                ready.insert(step);
            }
        }
        Problem { dependencies, workers: vec![None; 5], ready, time: 0 }
    }

    fn next_step(&mut self) -> Option<u8> {
        if let Some(&next) = self.ready.iter().next() {
            self.ready.remove(&next);
            Some(next)
        } else {
            None
        }
    }

    fn next_worker(&mut self) -> usize {
        loop {
            for i in 0 .. self.workers.len() {
                if self.workers[i].is_none() {
                    return i;
                }
            }
            self.advance_time();
        }
    }

    fn advance_time(&mut self) {
        self.time += 1;
        for worker in self.workers.iter_mut() {
            if worker.is_none() {
                continue;
            }
            worker.as_mut().unwrap().time -= 1;
            if worker.unwrap().time == 0 {
                let next = worker.unwrap().step;
                for (&step, dependencies) in self.dependencies.iter_mut() {
                    if dependencies.remove(&next) && dependencies.is_empty() {
                        self.ready.insert(step);
                    }
                }
                *worker = None;
            }
        }
    }

    fn solve(mut self) -> usize {
        loop {
            let i = self.next_worker();
            if let Some(step) = self.next_step() {
                self.workers[i] = Some(Work::new(step));
            } else {
                if self.workers.iter().any(|x| x.is_some()) {
                    self.advance_time();
                } else {
                    break;
                }
            }
        }
        self.time
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut dependencies: HashMap<u8, HashSet<u8>> = HashMap::new();
    for line in stdin.lock().lines() {
        let line: Vec<_> =
            line.unwrap().bytes().filter(|x| x.is_ascii_uppercase()).skip(1).collect();
        assert_eq!(line.len(), 2);
        dependencies.entry(line[1]).or_default().insert(line[0]);
        dependencies.entry(line[0]).or_default();
    }
    let problem = Problem::new(dependencies);
    println!("{}", problem.solve());
}
