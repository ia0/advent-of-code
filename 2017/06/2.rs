use std::collections::HashMap;
use std::io::BufRead;

fn find_max(banks: &Vec<u32>) -> usize {
    let mut max = 0;
    for cur in 1 .. banks.len() {
        if banks[cur] > banks[max] {
            max = cur;
        }
    }
    max
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut banks: Vec<u32> = line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = banks.len();
    let mut seen = HashMap::new();
    loop {
        let cur = seen.len();
        if let Some(prev) = seen.insert(banks.clone(), cur) {
            println!("{}", cur - prev);
            break;
        }
        let mut index = find_max(&banks);
        let mut amount = banks[index];
        banks[index] = 0;
        while amount > 0 {
            index = (index + 1) % n;
            banks[index] += 1;
            amount -= 1;
        }
    }
}
