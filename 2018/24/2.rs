use std::collections::HashSet;

#[derive(Clone)]
struct UnitType {
    hit_points: usize,
    weakness: HashSet<&'static str>,
    immunity: HashSet<&'static str>,
    damage: usize,
    attack: &'static str,
    initiative: usize,
}

impl UnitType {
    fn target_key(&self, attack: &'static str) -> u8 {
        if self.immunity.contains(attack) {
            panic!()
        } else if self.weakness.contains(attack) {
            0
        } else {
            1
        }
    }
}

#[derive(Clone)]
struct Group {
    count: usize,
    unit_type: UnitType,
}

impl Group {
    fn new(
        count: usize, hit_points: usize, damage: usize, attack: &'static str, initiative: usize,
    ) -> Group {
        Group {
            count,
            unit_type: UnitType {
                hit_points,
                weakness: HashSet::new(),
                immunity: HashSet::new(),
                damage,
                attack,
                initiative,
            },
        }
    }
}

impl Group {
    fn effective_power(&self) -> usize {
        self.count * self.unit_type.damage
    }

    fn selection_key(&self) -> (i64, i64) {
        (-(self.effective_power() as i64), -(self.unit_type.initiative as i64))
    }

    fn target_key(&self, attack: &'static str) -> (u8, (i64, i64)) {
        (self.unit_type.target_key(attack), self.selection_key())
    }

    fn suffer(&mut self, mut damage: usize, attack: &'static str) {
        if self.unit_type.weakness.contains(attack) {
            damage *= 2;
        }
        let killed = std::cmp::min(damage / self.unit_type.hit_points, self.count);
        self.count -= killed;
    }
}

#[derive(Clone)]
struct Army {
    groups: Vec<Group>,
    target: Vec<Option<usize>>,
    is_targeted: Vec<bool>,
}

impl Army {
    fn new() -> Army {
        Army { groups: Vec::new(), target: Vec::new(), is_targeted: Vec::new() }
    }

    fn begin_fight(&mut self) {
        self.is_targeted.resize(self.groups.len(), false);
        self.groups.sort_by_key(|group| group.selection_key());
    }

    fn selection(&mut self, enemy: &mut Army) {
        for group in self.groups.iter_mut() {
            let mut target: Option<usize> = None;
            for i in 0 .. enemy.groups.len() {
                if enemy.is_targeted[i] {
                    continue;
                }
                if enemy.groups[i].unit_type.immunity.contains(group.unit_type.attack) {
                    continue;
                }
                if target.is_none()
                    || enemy.groups[i].target_key(group.unit_type.attack)
                        < enemy.groups[target.unwrap()].target_key(group.unit_type.attack)
                {
                    if let Some(j) = target {
                        enemy.is_targeted[j] = false;
                    }
                    target = Some(i);
                    enemy.is_targeted[i] = true;
                }
            }
            self.target.push(target);
        }
    }

    fn end_fight(&mut self) {
        self.groups.retain(|group| group.count > 0);
        self.target.clear();
        self.is_targeted.clear();
    }

    fn remaining_units(&self) -> usize {
        self.groups.iter().map(|group| group.count).sum()
    }
}

#[derive(Clone)]
struct System {
    armies: [Army; 2],
}

impl System {
    fn boost(&self, boost: usize) -> System {
        let mut system = self.clone();
        for group in system.armies[0].groups.iter_mut() {
            group.unit_type.damage += boost;
        }
        system
    }

    fn solve(&mut self) {
        let mut prev_state = (self.armies[0].remaining_units(), self.armies[1].remaining_units());
        while !self.armies[0].groups.is_empty() && !self.armies[1].groups.is_empty() {
            self.armies[0].begin_fight();
            self.armies[1].begin_fight();

            let (left, right) = self.armies.split_at_mut(1);
            left[0].selection(&mut right[0]);
            right[0].selection(&mut left[0]);

            let mut attack_order = Vec::new();
            for i in 0 .. 2 {
                for j in 0 .. self.armies[i].groups.len() {
                    attack_order.push((i, j));
                }
            }
            attack_order
                .sort_by_key(|&(i, j)| -(self.armies[i].groups[j].unit_type.initiative as i64));
            for (i, j) in attack_order {
                let target = match self.armies[i].target[j] {
                    None => continue,
                    Some(target) => target,
                };
                let damage = self.armies[i].groups[j].effective_power();
                let attack = self.armies[i].groups[j].unit_type.attack;
                self.armies[1 - i].groups[target].suffer(damage, attack);
            }

            self.armies[0].end_fight();
            self.armies[1].end_fight();

            let state = (self.armies[0].remaining_units(), self.armies[1].remaining_units());
            if prev_state == state {
                break;
            }
            prev_state = state;
        }
    }

    fn immune_winner(&self) -> bool {
        !self.armies[0].groups.is_empty() && self.armies[1].groups.is_empty()
    }
}

fn main() {
    let mut armies = [Army::new(), Army::new()];
    armies[0].groups.push(Group::new(1514, 8968, 57, "bludgeoning", 9));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("cold");
    armies[0].groups.push(Group::new(2721, 6691, 22, "slashing", 15));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("cold");
    armies[0].groups.push(Group::new(1214, 10379, 69, "fire", 16));
    armies[0].groups.last_mut().unwrap().unit_type.immunity.insert("bludgeoning");
    armies[0].groups.push(Group::new(2870, 4212, 11, "radiation", 12));
    armies[0].groups.push(Group::new(1239, 5405, 37, "cold", 18));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("cold");
    armies[0].groups.push(Group::new(4509, 4004, 8, "slashing", 20));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("cold");
    armies[0].groups.last_mut().unwrap().unit_type.immunity.insert("radiation");
    armies[0].groups.push(Group::new(3369, 10672, 29, "cold", 11));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("slashing");
    armies[0].groups.push(Group::new(2890, 11418, 30, "cold", 8));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("fire");
    armies[0].groups.last_mut().unwrap().unit_type.immunity.insert("bludgeoning");
    armies[0].groups.push(Group::new(149, 7022, 393, "radiation", 13));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("slashing");
    armies[0].groups.push(Group::new(2080, 5786, 20, "fire", 7));
    armies[0].groups.last_mut().unwrap().unit_type.weakness.insert("fire");
    armies[0].groups.last_mut().unwrap().unit_type.immunity.insert("slashing");
    armies[0].groups.last_mut().unwrap().unit_type.immunity.insert("bludgeoning");
    armies[1].groups.push(Group::new(817, 47082, 115, "cold", 3));
    armies[1].groups.last_mut().unwrap().unit_type.immunity.insert("slashing");
    armies[1].groups.last_mut().unwrap().unit_type.immunity.insert("radiation");
    armies[1].groups.last_mut().unwrap().unit_type.immunity.insert("bludgeoning");
    armies[1].groups.push(Group::new(4183, 35892, 16, "bludgeoning", 1));
    armies[1].groups.push(Group::new(7006, 11084, 2, "fire", 2));
    armies[1].groups.push(Group::new(4804, 25411, 10, "cold", 14));
    armies[1].groups.push(Group::new(6262, 28952, 7, "slashing", 10));
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("fire");
    armies[1].groups.push(Group::new(628, 32906, 99, "radiation", 4));
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("slashing");
    armies[1].groups.push(Group::new(5239, 46047, 14, "bludgeoning", 6));
    armies[1].groups.last_mut().unwrap().unit_type.immunity.insert("fire");
    armies[1].groups.push(Group::new(1173, 32300, 53, "bludgeoning", 19));
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("cold");
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("slashing");
    armies[1].groups.push(Group::new(3712, 12148, 5, "slashing", 17));
    armies[1].groups.last_mut().unwrap().unit_type.immunity.insert("cold");
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("slashing");
    armies[1].groups.push(Group::new(334, 43582, 260, "cold", 5));
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("cold");
    armies[1].groups.last_mut().unwrap().unit_type.weakness.insert("fire");

    let system = System { armies };

    let mut max_boost = 1;
    loop {
        let mut system = system.boost(max_boost);
        system.solve();
        if system.immune_winner() {
            break;
        }
        max_boost *= 2;
    }

    let mut min_boost = 0;
    while max_boost - min_boost > 1 {
        let boost = (min_boost + max_boost) / 2;
        let mut system = system.boost(boost);
        system.solve();
        if system.immune_winner() {
            max_boost = boost;
        } else {
            min_boost = boost;
        }
    }

    let mut system = system.boost(max_boost);
    system.solve();
    println!("{}", system.armies[0].remaining_units());
}
