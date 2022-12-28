#![feature(drain_filter)]
#![feature(step_trait)]

use std::collections::{HashMap, HashSet};

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
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

pub const ADJACENT_PLUS: [Coord<i64>; 4] =
    [Coord { x: -1, y: 0 }, Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }, Coord { x: 0, y: -1 }];

pub const ADJACENT_STAR: [Coord<i64>; 8] = [
    Coord { x: -1, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: -1, y: 1 },
    Coord { x: 0, y: -1 },
    Coord { x: 0, y: 1 },
    Coord { x: 1, y: -1 },
    Coord { x: 1, y: 0 },
    Coord { x: 1, y: 1 },
];

impl Coord<i64> {
    pub fn contains(self, p: Coord<i64>) -> bool {
        0 <= p.x && p.x < self.x && 0 <= p.y && p.y < self.y
    }

    pub fn iter(self) -> impl Iterator<Item = Coord<i64>> {
        (0 .. self.x).map(move |x| (0 .. self.y).map(move |y| Coord { x, y })).flatten()
    }
}

pub fn print_set<T>(set: &HashSet<Coord<T>>, rev_y: bool)
where
    T: Default + Copy + std::hash::Hash + Ord + std::iter::Step,
{
    print_map(&set.iter().map(|&x| (x, '#')).collect(), rev_y, |&x| x);
}

pub fn print_map<T, V>(map: &HashMap<Coord<T>, V>, rev_y: bool, as_char: impl Fn(&V) -> char)
where
    T: Default + Copy + std::hash::Hash + Ord + std::iter::Step,
    std::ops::RangeInclusive<T>: Iterator<Item = T>,
{
    #[derive(Debug)]
    struct Frame<T> {
        min: Coord<T>,
        max: Coord<T>,
    }
    let init = match map.keys().next() {
        None => return,
        Some(x) => *x,
    };
    let frame = map.keys().fold(Frame { min: init, max: init }, |mut frame, coord| {
        frame.min.x = std::cmp::min(frame.min.x, coord.x);
        frame.min.y = std::cmp::min(frame.min.y, coord.y);
        frame.max.x = std::cmp::max(frame.max.x, coord.x);
        frame.max.y = std::cmp::max(frame.max.y, coord.y);
        frame
    });
    let mut y_axis: Box<dyn DoubleEndedIterator<Item = T>> = Box::new(frame.min.y ..= frame.max.y);
    if rev_y {
        y_axis = Box::new(y_axis.rev());
    }
    for y in y_axis {
        for x in frame.min.x ..= frame.max.x {
            print!("{}", map.get(&Coord { x, y }).map(|c| as_char(c)).unwrap_or(' '));
        }
        println!();
    }
}

impl<T: std::ops::Add<T, Output = T>> std::ops::Add<Coord<T>> for Coord<T> {
    type Output = Coord<T>;

    fn add(self, rhs: Coord<T>) -> Coord<T> {
        Coord { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: std::ops::AddAssign<T>> std::ops::AddAssign<Coord<T>> for Coord<T> {
    fn add_assign(&mut self, rhs: Coord<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: std::ops::Sub<T, Output = T>> std::ops::Sub<Coord<T>> for Coord<T> {
    type Output = Coord<T>;

    fn sub(self, rhs: Coord<T>) -> Coord<T> {
        Coord { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Copy + std::ops::Mul<T, Output = T>> std::ops::Mul<T> for Coord<T> {
    type Output = Coord<T>;

    fn mul(self, rhs: T) -> Coord<T> {
        Coord { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Copy + std::ops::Mul<T, Output = T>> std::ops::Mul<Coord<T>> for Coord<T> {
    type Output = Coord<T>;

    fn mul(self, rhs: Coord<T>) -> Coord<T> {
        Coord { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
