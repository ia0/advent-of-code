use std::io::BufRead;

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

struct Point {
    position: Coord,
    velocity: Coord,
}

struct Frame {
    min: Coord,
    max: Coord,
}

fn get_frame(points: &[Point]) -> Frame {
    let mut min = points[0].position;
    let mut max = points[0].position;
    for point in points {
        min.x = std::cmp::min(min.x, point.position.x);
        min.y = std::cmp::min(min.y, point.position.y);
        max.x = std::cmp::max(max.x, point.position.x);
        max.y = std::cmp::max(max.y, point.position.y);
    }
    Frame { min, max }
}

fn get_size(points: &[Point]) -> usize {
    let Frame { min, max } = get_frame(points);
    (max.x - min.x) as usize + (max.y - min.y) as usize
}

fn advance(points: &mut [Point]) {
    for point in points {
        point.position.x += point.velocity.x;
        point.position.y += point.velocity.y;
    }
}

fn rewind(points: &mut [Point]) {
    for point in points {
        point.position.x -= point.velocity.x;
        point.position.y -= point.velocity.y;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut points = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let words: Vec<Option<i32>> =
            line.split(|x| ['<', '>', ','].contains(&x)).map(|x| x.trim().parse().ok()).collect();
        points.push(Point {
            position: Coord { x: words[1].unwrap(), y: words[2].unwrap() },
            velocity: Coord { x: words[4].unwrap(), y: words[5].unwrap() },
        });
    }
    let mut size = get_size(&points);
    let mut time = 0;
    loop {
        advance(&mut points);
        let new_size = get_size(&points);
        if new_size > size {
            rewind(&mut points);
            break;
        }
        time += 1;
        size = new_size;
    }
    println!("{}", time);
}
