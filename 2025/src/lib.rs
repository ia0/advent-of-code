#![feature(step_trait)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::Write;
use std::ops::{Deref, Range};

use anyhow::{Context, Result, bail};
use num::{Integer, Signed};

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

/// Parses an input string as whitespace-separated integers.
pub fn parse_ints(input: &str) -> Result<Vec<i64>> {
    input.split_whitespace().map(|x| Ok(x.parse()?)).collect()
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

/// Returns (gcd(a, b), s, t) s.t. `gcd(a, b) == a * s + b * t`.
pub fn egcd<T: Clone + Signed + Integer>(a: T, b: T) -> (T, T, T) {
    if a < T::zero() || b < T::zero() {
        let (g, s, t) = egcd(a.abs(), b.abs());
        return (g, a.signum() * s, b.signum() * t);
    }
    let mut rx = a;
    let mut sx = T::one();
    let mut tx = T::zero();
    let mut ry = b;
    let mut sy = T::zero();
    let mut ty = T::one();
    while T::zero() < ry {
        let q = rx.clone() / ry.clone();
        rx = rx - q.clone() * ry.clone();
        sx = sx - q.clone() * sy.clone();
        tx = tx - q * ty.clone();
        std::mem::swap(&mut rx, &mut ry);
        std::mem::swap(&mut sx, &mut sy);
        std::mem::swap(&mut tx, &mut ty);
    }
    (rx, sx, tx)
}

#[test]
fn egcd_ok() {
    assert_eq!(egcd(2, 3), (1, -1, 1));
    assert_eq!(egcd(-2, 3), (1, 1, 1));
    assert_eq!(egcd(2, -3), (1, -1, -1));
    assert_eq!(egcd(-2, -3), (1, 1, -1));
    assert_eq!(egcd(3, 2), (1, 1, -1));
    assert_eq!(egcd(2, 4), (2, 1, 0));
    assert_eq!(egcd(6, 8), (2, -1, 1));
    assert_eq!(egcd(6, 1), (1, 0, 1));
    assert_eq!(egcd(1, 6), (1, 1, 0));
    assert_eq!(egcd(6, 0), (6, 1, 0));
    assert_eq!(egcd(0, 6), (6, 0, 1));
}

/// Returns (r, m) s.t. `x = r [m]` given `x = r_i [m_i]`.
pub fn crt(rms: &[(i64, i64)]) -> (i64, i64) {
    let mut rm = rms[0];
    for &rms in &rms[1 ..] {
        let (r, s, t) = egcd(rm.1, rms.1);
        assert_eq!(r, 1);
        rm.0 = rm.0 * t * rms.1 + rms.0 * s * rm.1;
        rm.1 *= rms.1;
        rm.0 = rm.0.rem_euclid(rm.1);
    }
    rm
}

#[test]
fn crt_ok() {
    assert_eq!(crt(&[(1, 2), (2, 3)]), (5, 6));
    assert_eq!(crt(&[(1, 2), (2, 3), (1, 5)]), (11, 30));
}

pub fn order_pair<T: Ord>(x: T, y: T) -> (T, T) {
    if x <= y { (x, y) } else { (y, x) }
}

pub fn order_pair_mut<T: Ord>(x: &mut T, y: &mut T) {
    if y < x {
        std::mem::swap(x, y);
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Default, Copy, Clone)]
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

    pub fn contains(self, coord: Coord) -> bool {
        (self.min.x ..= self.max.x).contains(&coord.x)
            && (self.min.y ..= self.max.y).contains(&coord.y)
    }

    pub fn iter(self) -> impl Iterator<Item = Coord> {
        (self.min.x ..= self.max.x)
            .flat_map(move |x| (self.min.y ..= self.max.y).map(move |y| Coord { x, y }))
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(self) -> usize {
        ((self.max.y - self.min.y + 1) * (self.max.x - self.min.x + 1)) as usize
    }
}

pub const NORTH: Coord = Coord { x: 0, y: -1 };
pub const SOUTH: Coord = Coord { x: 0, y: 1 };
pub const EAST: Coord = Coord { x: 1, y: 0 };
pub const WEST: Coord = Coord { x: -1, y: 0 };

pub const ADJACENT_PLUS: [Coord; 4] = [NORTH, SOUTH, EAST, WEST];

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
    pub fn parse(input: &str, sep: &str) -> Result<Coord> {
        let (x, y) = input.split_once(sep).context("missing separator")?;
        Ok(Coord { x: x.parse()?, y: y.parse()? })
    }

    pub fn parse_dir(byte: u8) -> Result<Coord> {
        Ok(match byte {
            b'<' => WEST,
            b'>' => EAST,
            b'^' => NORTH,
            b'v' => SOUTH,
            _ => bail!("invalid direction {:?}", byte as char),
        })
    }

    pub fn contains(self, p: Coord) -> bool {
        0 <= p.x && p.x < self.x && 0 <= p.y && p.y < self.y
    }

    pub fn iter(self) -> impl Iterator<Item = Coord> {
        (0 .. self.x).flat_map(move |x| (0 .. self.y).map(move |y| Coord { x, y }))
    }

    pub fn left(self) -> Self {
        Coord { x: self.y, y: -self.x }
    }

    pub fn right(self) -> Self {
        Coord { x: -self.y, y: self.x }
    }

    pub fn dist_plus(self, other: Coord) -> usize {
        (self - other).len_plus()
    }

    pub fn len_plus(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    pub fn signum(self) -> Coord {
        Coord { x: self.x.signum(), y: self.y.signum() }
    }
}

pub fn shortest_dist_plus(walls: &HashSet<Coord>, a: Coord, b: Coord) -> Option<usize> {
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), a));
    let mut visited = HashSet::new();
    while let Some((Reverse(d), a)) = todo.pop() {
        if !visited.insert(a) {
            continue;
        }
        if a == b {
            return Some(d);
        }
        for x in ADJACENT_PLUS {
            let a = a + x;
            if walls.contains(&a) {
                continue;
            }
            todo.push((Reverse(d + 1), a));
        }
    }
    None
}

pub fn print_set(output: impl Write, set: &HashSet<Coord>, rev_y: bool) -> Result<()> {
    print_map(output, &set.iter().map(|&x| (x, '#')).collect(), rev_y, |&x| x)
}

pub fn print_map<V>(
    mut output: impl Write, map: &HashMap<Coord, V>, rev_y: bool, mut as_char: impl Fn(&V) -> char,
) -> Result<()> {
    let frame = Frame::new(map.keys().cloned()).unwrap();
    let mut y_axis: Box<dyn DoubleEndedIterator<Item = i64>> =
        Box::new(frame.min.y ..= frame.max.y);
    if rev_y {
        y_axis = Box::new(y_axis.rev());
    }
    for y in y_axis {
        for x in frame.min.x ..= frame.max.x {
            write!(output, "{}", map.get(&Coord { x, y }).map(&mut as_char).unwrap_or(' '))?;
        }
        writeln!(output)?;
    }
    Ok(())
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

impl std::ops::RemAssign<Coord> for Coord {
    fn rem_assign(&mut self, rhs: Coord) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

impl std::ops::Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Coord { x: -self.x, y: -self.y }
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

impl Deref for Intervals {
    type Target = [Range<i64>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
        self.0.retain_mut(|x| {
            x.start = std::cmp::max(x.start, range.start);
            x.end = std::cmp::min(x.end, range.end);
            !x.is_empty()
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

#[allow(clippy::single_range_in_vec_init)]
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
