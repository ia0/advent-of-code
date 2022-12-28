use std::io::BufRead;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, p: Point) -> i32 {
        (p.x - self.x).abs() + (p.y - self.y).abs()
    }
}

struct Frame {
    points: Vec<Point>,
    min: Point,
    max: Point,
}

impl Frame {
    fn new(x: i32, y: i32) -> Frame {
        let p = Point { x, y };
        Frame { points: Vec::new(), min: p, max: p }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.points.push(Point { x, y });
        self.min.x = std::cmp::min(self.min.x, x);
        self.max.x = std::cmp::max(self.max.x, x);
        self.min.y = std::cmp::min(self.min.y, y);
        self.max.y = std::cmp::max(self.max.y, y);
    }

    fn closest(&self, p: Point) -> Option<usize> {
        (0 .. self.points.len()).min_by_key(|&i| self.points[i].dist(p))
    }

    fn done(&mut self) -> usize {
        let mut size = vec![Some(0); self.points.len()];
        for x in self.min.x ..= self.max.x {
            for y in self.min.y ..= self.max.y {
                if let Some(i) = self.closest(Point { x, y }) {
                    if x == self.min.x || x == self.max.x || y == self.min.y || y == self.max.y {
                        size[i] = None;
                    }
                    if let Some(ref mut v) = size[i] {
                        *v += 1;
                    }
                }
            }
        }
        size.into_iter().filter_map(|x| x).max().unwrap()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut frame = None;
    for line in stdin.lock().lines() {
        let xy: Vec<i32> = line.unwrap().split(", ").map(|x| x.parse().unwrap()).collect();
        assert_eq!(xy.len(), 2);
        let (x, y) = (xy[0], xy[1]);
        match frame {
            None => frame = Some(Frame::new(x, y)),
            Some(ref mut frame) => frame.add(x, y),
        }
    }
    println!("{}", frame.unwrap().done());
}
