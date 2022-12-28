use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufRead;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Team {
    Gobelin,
    Elf,
}
use Team::*;

impl Team {
    fn opposite(self) -> Team {
        match self {
            Gobelin => Elf,
            Elf => Gobelin,
        }
    }
}

enum Cell {
    Wall,
    Empty,
    Unit { team: Team, hit_points: u32, next_round: u32 },
}
use Cell::*;

impl Cell {
    fn team(&self) -> Option<Team> {
        match self {
            Unit { team, .. } => Some(*team),
            _ => None,
        }
    }

    fn has_team(&self, x: Team) -> bool {
        self.team() == Some(x)
    }

    fn is_empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }

    fn is_dead(&mut self) -> bool {
        match self {
            Unit { hit_points, .. } => {
                *hit_points = hit_points.saturating_sub(3);
                *hit_points <= 0
            }
            _ => false,
        }
    }

    fn hit_points(&self) -> Option<u32> {
        match self {
            Unit { hit_points, .. } => Some(*hit_points),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}
use Dir::*;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

fn is_target(cave: &[Vec<Cell>], pos: Coord, team: Team) -> bool {
    let opposite_team = team.opposite();
    return cave[pos.x - 1][pos.y].has_team(opposite_team)
        || cave[pos.x][pos.y - 1].has_team(opposite_team)
        || cave[pos.x][pos.y + 1].has_team(opposite_team)
        || cave[pos.x + 1][pos.y].has_team(opposite_team);
}

fn add_next_step(
    next_steps: &mut VecDeque<(Dir, usize, Coord)>, dir: Dir, dist: usize, pos: Coord,
) {
    let dist = dist + 1;
    next_steps.push_back((dir, dist, Coord { x: pos.x - 1, y: pos.y }));
    next_steps.push_back((dir, dist, Coord { x: pos.x, y: pos.y - 1 }));
    next_steps.push_back((dir, dist, Coord { x: pos.x, y: pos.y + 1 }));
    next_steps.push_back((dir, dist, Coord { x: pos.x + 1, y: pos.y }));
}

fn compute_step(cave: &[Vec<Cell>], pos: Coord, team: Team) -> Option<Dir> {
    let mut fastest_step = HashMap::new();
    let mut next_steps = VecDeque::new();
    next_steps.push_back((Up, 1, Coord { x: pos.x - 1, y: pos.y }));
    next_steps.push_back((Left, 1, Coord { x: pos.x, y: pos.y - 1 }));
    next_steps.push_back((Right, 1, Coord { x: pos.x, y: pos.y + 1 }));
    next_steps.push_back((Down, 1, Coord { x: pos.x + 1, y: pos.y }));
    let mut closest_dist = None;
    let mut closest_pos = Vec::new();
    while let Some((dir, dist, pos)) = next_steps.pop_front() {
        if let Some(closest_dist) = closest_dist {
            if dist > closest_dist {
                break;
            }
        }
        match cave[pos.x][pos.y] {
            Wall => continue,
            Unit { team: x, .. } => {
                assert!(team == x);
                continue;
            }
            Empty => (),
        }
        let visited = fastest_step.contains_key(&pos);
        let entry = fastest_step.entry(pos).or_insert((dist, dir));
        if dist < entry.0 {
            entry.0 = dist;
            entry.1 = dir;
        } else if dist == entry.0 && dir < entry.1 {
            entry.1 = dir;
        }
        if is_target(cave, pos, team) {
            match closest_dist {
                None => closest_dist = Some(dist),
                Some(x) => assert_eq!(dist, x),
            }
            closest_pos.push(pos);
        } else if !visited {
            add_next_step(&mut next_steps, dir, dist, pos);
        }
    }
    closest_pos.sort();
    closest_pos.first().map(|pos| fastest_step.get(pos).unwrap().1)
}

fn compute_enemy(cave: &[Vec<Cell>], pos: Coord, team: Team) -> Option<Coord> {
    let mut targets = Vec::new();
    targets.push(Coord { x: pos.x - 1, y: pos.y });
    targets.push(Coord { x: pos.x, y: pos.y - 1 });
    targets.push(Coord { x: pos.x, y: pos.y + 1 });
    targets.push(Coord { x: pos.x + 1, y: pos.y });
    let targets: Vec<Coord> =
        targets.into_iter().filter(|pos| cave[pos.x][pos.y].has_team(team.opposite())).collect();
    let min_hit_points =
        match targets.iter().map(|pos| cave[pos.x][pos.y].hit_points().unwrap()).min() {
            None => return None,
            Some(x) => x,
        };
    targets
        .into_iter()
        .filter(|pos| cave[pos.x][pos.y].hit_points().unwrap() == min_hit_points)
        .next()
}

fn main() {
    let stdin = std::io::stdin();
    let mut cave = Vec::new();
    let mut num_elf = 0;
    let mut num_gobelin = 0;
    for line in stdin.lock().lines() {
        let mut row = Vec::new();
        for c in line.unwrap().chars() {
            row.push(match c {
                '#' => Wall,
                '.' => Empty,
                'E' => {
                    num_elf += 1;
                    Unit { team: Elf, hit_points: 200, next_round: 0 }
                }
                'G' => {
                    num_gobelin += 1;
                    Unit { team: Gobelin, hit_points: 200, next_round: 0 }
                }
                _ => panic!(),
            });
        }
        cave.push(row);
        assert_eq!(cave[0].len(), cave.last().unwrap().len());
    }
    let mut round = 0;
    loop {
        for x in 0 .. cave.len() {
            for y in 0 .. cave[x].len() {
                if num_elf == 0 || num_gobelin == 0 {
                    let sum = cave
                        .iter()
                        .map(|row| row.iter().filter_map(Cell::hit_points).sum::<u32>())
                        .sum::<u32>();
                    println!("{}", round * sum);
                    return;
                }
                let team = match cave[x][y] {
                    Wall | Empty => continue,
                    Unit { next_round, .. } if next_round != round => {
                        assert_eq!(next_round, round + 1);
                        continue;
                    }
                    Unit { team, ref mut next_round, .. } => {
                        *next_round += 1;
                        team
                    }
                };
                let mut pos = Coord { x, y };
                if !is_target(&cave, pos, team) {
                    let step = match compute_step(&cave, pos, team) {
                        None => continue,
                        Some(step) => step,
                    };
                    let mut cell = Empty;
                    std::mem::swap(&mut cell, &mut cave[pos.x][pos.y]);
                    match step {
                        Up => pos.x -= 1,
                        Left => pos.y -= 1,
                        Right => pos.y += 1,
                        Down => pos.x += 1,
                    };
                    std::mem::swap(&mut cell, &mut cave[pos.x][pos.y]);
                    assert!(cell.is_empty());
                }
                if let Some(enemy_pos) = compute_enemy(&cave, pos, team) {
                    if cave[enemy_pos.x][enemy_pos.y].is_dead() {
                        match cave[enemy_pos.x][enemy_pos.y].team().unwrap() {
                            Elf => num_elf -= 1,
                            Gobelin => num_gobelin -= 1,
                        }
                        cave[enemy_pos.x][enemy_pos.y] = Empty;
                    }
                }
            }
        }
        round += 1;
    }
}
