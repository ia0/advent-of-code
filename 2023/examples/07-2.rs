#![feature(slice_group_by)]

use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: i64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        rank(self.cards).cmp(&rank(other.cards)).then_with(|| self.cards.cmp(&other.cards))
    }
}

fn rank(mut cards: [u8; 5]) -> u16 {
    cards.sort();
    let jokers = cards.iter().take_while(|&&x| x == 0).count();
    let mut count: Vec<_> =
        cards[jokers ..].group_by(|x, y| x == y).map(|x| x.len() as u8).collect();
    count.sort();
    count.reverse();
    count.resize(2, 0);
    count[0] += jokers as u8;
    u16::from_be_bytes(count.try_into().unwrap())
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (cards, bid) = input.split_once(' ').context("no space")?;
        let mut cards: [u8; 5] = cards.as_bytes().try_into().context("not 5 cards")?;
        for card in cards.iter_mut() {
            *card = match *card {
                b'J' => 0,
                x @ b'2' ..= b'9' => x - b'1',
                b'T' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                x => bail!("{x:02x} not a card"),
            };
        }
        let bid = bid.parse().context("bid not a number")?;
        Ok(Hand { cards, bid })
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut hands: Vec<Hand> =
        BufReader::new(input).lines().map(|x| x?.parse()).collect::<Result<_>>()?;
    hands.sort();
    let total: i64 = hands.iter().enumerate().map(|(i, x)| (i as i64 + 1) * x.bid).sum();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/07.txt") == "250757288\n");
