use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
struct State {
    cost: i64,
    rooms: Vec<Vec<char>>,
    hallway: Vec<char>,
}

impl State {
    fn solved(&self) -> bool {
        self.rooms[0].iter().all(|&c| c == 'A')
            && self.rooms[1].iter().all(|&c| c == 'B')
            && self.rooms[2].iter().all(|&c| c == 'C')
            && self.rooms[3].iter().all(|&c| c == 'D')
            && self.hallway.iter().all(|&c| c == '.')
    }

    fn path_ok(&self, from: i8, to: i8) -> bool {
        let c1 = if from < to { from + 1 } else { from - 1 };
        let c2 = to;
        (c1.min(c2)..=c1.max(c2))
            .map(|c| self.hallway[c as usize])
            .all(|c| c == '.')
    }

    fn hash_val(&self) -> (i64, i64) {
        let mut v = 0;
        let b = 3;
        for c in self.hallway.iter() {
            if *c == '.' {
                v <<= b;
            } else {
                v = (v << b) + (*c as i64 - 'A' as i64 + 1);
            }
        }

        let mut v2 = 0;
        for r in self.rooms.iter() {
            for c in r.iter() {
                v2 = (v2 << b) + (*c as i64 - 'A' as i64 + 1);
            }
            for _ in 0..4 - r.len() {
                v2 <<= b;
            }
        }
        (v, v2)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn min_cost(s: &State) -> i64 {
    let mut q: BinaryHeap<State> = BinaryHeap::from([s.clone()]);
    let mut cost: HashMap<(i64, i64), i64> = HashMap::new();
    let mut min_cost = i64::MAX;

    while let Some(curr_s) = q.pop() {
        if curr_s.solved() {
            min_cost = min_cost.min(curr_s.cost);
            // println!("Solved: {:?} {:?}" , curr_s, curr_s.hash_val());
            continue;
        }

        // from rooms to hallway
        for n in 0..curr_s.rooms.len() {
            if curr_s.rooms[n].is_empty() {
                continue;
            }

            for (i, c) in curr_s.hallway.iter().enumerate() {
                if i == 2 || i == 4 || i == 6 || i == 8 {
                    continue;
                }
                if *c == '.' && curr_s.path_ok((n as i8 + 1) * 2, i as i8) {
                    let mut new_s = curr_s.clone();
                    let c2 = new_s.rooms[n].pop().unwrap();
                    new_s.hallway[i] = c2;
                    let hash = new_s.hash_val();

                    let halldist = (i as i64 - (n as i64 + 1) * 2).abs();
                    let roomdist = 4 - new_s.rooms[n].len() as i64;
                    new_s.cost += (roomdist + halldist) * i64::pow(10, c2 as u32 - 'A' as u32);
                    if let Some(old_cost) = cost.get(&hash) {
                        if new_s.cost < *old_cost {
                            cost.insert(hash, new_s.cost);
                            q.push(new_s);
                        }
                    } else {
                        cost.insert(hash, new_s.cost);
                        q.push(new_s);
                    }
                }
            }
        }

        // from hallway to rooms
        for (i, c) in curr_s.hallway.iter().enumerate() {
            if *c != '.' {
                let r_num = *c as usize - 'A' as usize;
                if curr_s.rooms[r_num].iter().all(|c1| c1 == c)
                    && curr_s.rooms[r_num].len() < 4
                    && curr_s.path_ok(i as i8, (r_num as i8 + 1) * 2)
                {
                    let mut new_s = curr_s.clone();
                    new_s.rooms[r_num].push(*c);
                    new_s.hallway[i] = '.';
                    let hash = new_s.hash_val();

                    let halldist = (i as i64 - (r_num as i64 + 1) * 2).abs();
                    let roomdist = 5 - new_s.rooms[r_num].len() as i64;
                    new_s.cost += (roomdist + halldist) * i64::pow(10, *c as u32 - 'A' as u32);
                    if let Some(old_cost) = cost.get(&hash) {
                        if new_s.cost < *old_cost {
                            cost.insert(hash, new_s.cost);
                            q.push(new_s);
                        }
                    } else {
                        cost.insert(hash, new_s.cost);
                        q.push(new_s);
                    }
                }
            }
        }
    }
    min_cost
}

pub fn main() {
    let mut s = State {
        cost: 0,
        rooms: vec![vec![], vec![], vec![], vec![]],
        hallway: vec!['.'; 11],
    };
    // for (_row, line) in include_str!("input23_test.txt").lines().enumerate() {
    for (_row, line) in include_str!("../../inputPart2.txt").lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'A' | 'B' | 'C' | 'D' => s.rooms[col / 2 - 1].insert(0, c),
                _ => (),
            }
        }
    }
    println!(">>> {}", min_cost(&s));
}
