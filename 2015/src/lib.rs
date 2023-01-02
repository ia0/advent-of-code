#![feature(drain_filter)]
#![feature(step_trait)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Range;

#[macro_export]
macro_rules! main {
    ($solve:ident($input:expr)) => {
        fn main() -> anyhow::Result<()> {
            $solve(std::fs::File::open($input)?, std::io::stdout())?;
            Ok(())
        }
    };
    ($solve:ident($input:expr) == $output:expr) => {
        $crate::main!($solve($input));

        #[test]
        fn test() {
            let mut output = Vec::new();
            $solve(std::fs::File::open($input).unwrap(), &mut output).unwrap();
            assert_eq!(String::from_utf8(output).unwrap(), $output);
        }
    };
}

/// Returns the smallest `x` such that `f(x)` is `true`.
///
/// The function `f` must be monotonous and transition from `false` to `true`.
pub fn binary_search(f: impl Fn(usize) -> bool) -> usize {
    let mut min = 0;
    if f(min) {
        return min;
    }
    let mut max = 1;
    while !f(max) {
        max *= 2;
    }
    while min + 1 < max {
        let mid = (min + max) / 2;
        if f(mid) {
            max = mid;
        } else {
            min = mid;
        }
    }
    max
}

#[test]
fn binary_search_ok() {
    fn test(k: usize) {
        assert_eq!(binary_search(|x| x >= k), k);
    }
    for i in 0 .. 100 {
        test(i);
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct Frame {
    pub min: Coord,
    pub max: Coord,
}

impl Frame {
    pub fn new(mut iter: impl Iterator<Item = Coord>) -> Option<Self> {
        let init = iter.next()?;
        Some(iter.fold(Frame { min: init, max: init }, |mut frame, coord| {
            frame.min.x = std::cmp::min(frame.min.x, coord.x);
            frame.min.y = std::cmp::min(frame.min.y, coord.y);
            frame.max.x = std::cmp::max(frame.max.x, coord.x);
            frame.max.y = std::cmp::max(frame.max.y, coord.y);
            frame
        }))
    }
}

pub const ADJACENT_PLUS: [Coord; 4] =
    [Coord { x: -1, y: 0 }, Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }, Coord { x: 0, y: -1 }];

pub const ADJACENT_STAR: [Coord; 8] = [
    Coord { x: -1, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: -1, y: 1 },
    Coord { x: 0, y: -1 },
    Coord { x: 0, y: 1 },
    Coord { x: 1, y: -1 },
    Coord { x: 1, y: 0 },
    Coord { x: 1, y: 1 },
];

impl Coord {
    pub fn contains(self, p: Coord) -> bool {
        0 <= p.x && p.x < self.x && 0 <= p.y && p.y < self.y
    }

    pub fn iter(self) -> impl Iterator<Item = Coord> {
        (0 .. self.x).flat_map(move |x| (0 .. self.y).map(move |y| Coord { x, y }))
    }
}

pub fn print_set<T>(set: &HashSet<Coord>, rev_y: bool)
where
    T: Default + Copy + std::hash::Hash + Ord + std::iter::Step,
{
    print_map(&set.iter().map(|&x| (x, '#')).collect(), rev_y, |&x| x);
}

pub fn print_map<V>(map: &HashMap<Coord, V>, rev_y: bool, mut as_char: impl Fn(&V) -> char) {
    let frame = Frame::new(map.keys().cloned()).unwrap();
    let mut y_axis: Box<dyn DoubleEndedIterator<Item = i64>> =
        Box::new(frame.min.y ..= frame.max.y);
    if rev_y {
        y_axis = Box::new(y_axis.rev());
    }
    for y in y_axis {
        for x in frame.min.x ..= frame.max.x {
            print!("{}", map.get(&Coord { x, y }).map(&mut as_char).unwrap_or(' '));
        }
        println!();
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Coord {
        Coord { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Mul<i64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i64) -> Coord {
        Coord { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::ops::Mul<Coord> for Coord {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Coord {
        Coord { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

/// Return the top N elements of an iterator as an iterator.
pub fn topn<T: Ord>(n: usize, iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut best = BinaryHeap::new();
    for item in iter {
        best.push(Reverse(item));
        if best.len() > n {
            best.pop();
        }
    }
    best.into_iter().map(|x| x.0)
}

#[derive(Default)]
pub struct Intervals(Vec<Range<i64>>);

impl Intervals {
    pub fn insert(&mut self, range: Range<i64>) {
        let pos = self.0.iter().position(|x| range.start <= x.start).unwrap_or(self.0.len());
        self.0.insert(pos, range);
        self.merge();
    }

    fn merge(&mut self) {
        let mut i = 1;
        while i < self.0.len() {
            if self.0[i].start <= self.0[i - 1].end {
                self.0[i - 1].end = std::cmp::max(self.0[i - 1].end, self.0[i].end);
                self.0.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn intersect(&mut self, range: Range<i64>) {
        self.0.drain_filter(|x| {
            x.start = std::cmp::max(x.start, range.start);
            x.end = std::cmp::min(x.end, range.end);
            x.is_empty()
        });
    }

    pub fn contains(&self, x: i64) -> bool {
        self.0.iter().any(|r| r.contains(&x))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|x| x.end - x.start).sum::<i64>() as usize
    }
}

#[test]
fn intervals_insert() {
    #[track_caller]
    fn test(xs: &[Range<i64>], r: &[Range<i64>]) {
        let mut i = Intervals::default();
        for x in xs {
            i.insert(x.clone());
        }
        assert_eq!(&i.0, r);
    }
    test(&[], &[]);
    test(&[0 .. 10], &[0 .. 10]);
    test(&[0 .. 10, 20 .. 30], &[0 .. 10, 20 .. 30]);
    test(&[20 .. 30, 0 .. 10], &[0 .. 10, 20 .. 30]);
    test(&[0 .. 10, 20 .. 30, 5 .. 15], &[0 .. 15, 20 .. 30]);
    test(&[0 .. 10, 20 .. 30, 15 .. 25], &[0 .. 10, 15 .. 30]);
    test(&[0 .. 10, 20 .. 30, 10 .. 15], &[0 .. 15, 20 .. 30]);
    test(&[0 .. 10, 20 .. 30, 15 .. 20], &[0 .. 10, 15 .. 30]);
    test(&[0 .. 10, 20 .. 30, 10 .. 20], &[0 .. 30]);
    test(&[0 .. 10, 20 .. 30, 5 .. 25], &[0 .. 30]);
    test(&[0 .. 10, 20 .. 30, 35 .. 40], &[0 .. 10, 20 .. 30, 35 .. 40]);
    test(&[0 .. 10, 20 .. 30, 30 .. 40], &[0 .. 10, 20 .. 40]);
    test(&[0 .. 10, 20 .. 30, 25 .. 40], &[0 .. 10, 20 .. 40]);
}
