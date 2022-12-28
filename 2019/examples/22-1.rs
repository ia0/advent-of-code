use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Technique {
    Reverse,
    Increment(i32),
    Cut(i32),
}

impl FromStr for Technique {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Technique, Box<dyn Error>> {
        if input == "deal into new stack" {
            return Ok(Technique::Reverse);
        }
        if let Some(value) = input.strip_prefix("deal with increment ") {
            return Ok(Technique::Increment(value.parse()?));
        }
        if let Some(value) = input.strip_prefix("cut ") {
            return Ok(Technique::Cut(value.parse()?));
        }
        panic!("{}", input)
    }
}

impl Technique {
    fn apply(self, cards: &mut [i64]) {
        let n = cards.len();
        match self {
            Technique::Reverse => cards.reverse(),
            Technique::Increment(value) => {
                let value = usize::try_from(value).unwrap() % n;
                let mut new_cards = vec![-1; n];
                for i in 0 .. n {
                    new_cards[((i % n) * value) % n] = cards[i];
                }
                cards.copy_from_slice(&new_cards);
                assert!(cards.iter().all(|&x| x >= 0));
            }
            Technique::Cut(value) => {
                let value = usize::try_from(value.rem_euclid(n as i32)).unwrap();
                cards.rotate_left(value);
            }
        }
    }
}

fn main() {
    let input = File::open("examples/22.txt").unwrap();
    let shuffle: Vec<Technique> =
        BufReader::new(input).lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let mut cards: Vec<i64> = (0 .. 10007).collect();
    for technique in shuffle {
        technique.apply(&mut cards);
    }
    println!("{}", cards.iter().position(|&x| x == 2019).unwrap());
}
