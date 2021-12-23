#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
    No,
}

use std::ops::{Index, IndexMut};

use Amphipod::*;

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => Self::No,
        }
    }
}

impl Default for Amphipod {
    fn default() -> Self {
        Self::No
    }
}

impl Amphipod {
    fn idx(&self) -> Option<usize> {
        match self {
            A => Some(0),
            B => Some(1),
            C => Some(2),
            D => Some(3),
            No => None,
        }
    }
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let No = self {
            write!(f, ".")
        } else {
            write!(f, "{:?}", self)
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Positions {
    storage: [Amphipod; 7],
    rooms: [[Amphipod; 2]; 4],
}

impl Index<&Pos> for Positions {
    type Output = Amphipod;

    fn index(&self, index: &Pos) -> &Self::Output {
        match index {
            Pos::Storage(i) => &self.storage[*i],
            Pos::Room(r, i) => &self.rooms[*r][*i],
        }
    }
}

impl IndexMut<&Pos> for Positions {
    fn index_mut(&mut self, index: &Pos) -> &mut Self::Output {
        match index {
            Pos::Storage(i) => &mut self.storage[*i],
            Pos::Room(r, i) => &mut self.rooms[*r][*i],
        }
    }
}

impl std::fmt::Display for Positions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.storage[0],
            self.storage[1],
            self.storage[2],
            self.storage[3],
            self.storage[4],
            self.storage[5],
            self.storage[6]
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0]
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.rooms[0][1], self.rooms[1][1], self.rooms[2][1], self.rooms[3][1]
        )?;
        writeln!(f, "  #########")
    }
}

const FINAL_POS: Positions = Positions {
    storage: [Amphipod::No; 7],
    rooms: [[A, A], [B, B], [C, C], [D, D]],
};

fn successors((p, _): &(Positions, usize)) -> Vec<((Positions, usize), usize)> {
    let mut res = Vec::new();
    for (s_pos, v) in p.storage.iter().enumerate().filter(|(_, x)| *x != &No) {
        let s_pos = Pos::Storage(s_pos);
        for pos in possible_postions(p, s_pos) {
            let mut new = p.clone();
            new[&pos] = *v;
            new[&s_pos] = No;
            let cost = s_pos.cost(&pos, v);
            res.push(((new, cost), cost))
        }
    }
    for (r, x) in p.rooms.iter().enumerate() {
        for (i, v) in x.iter().enumerate().filter(|(_, x)| *x != &No) {
            let pos = Pos::Room(r, i);
            for new_pos in possible_postions(p, pos) {
                // println!("{} -> {}", pos, new_pos);
                let mut new = p.clone();
                new[&new_pos] = *v;
                new[&pos] = No;
                let cost = pos.cost(&new_pos, v);
                res.push(((new, cost), cost))
            }
        }
    }

    res
}

#[derive(Debug, Clone, Copy)]
enum Pos {
    Storage(usize),
    Room(usize, usize),
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pos::Storage(i) => write!(f, "S{}", i),
            Pos::Room(r, i) => write!(f, "R{};{}", r, i),
        }
    }
}

impl Pos {
    fn as_coords(&self) -> (usize, usize) {
        match self {
            Pos::Storage(i) => (
                *i + if *i < 2 {
                    0
                } else if *i < 3 {
                    1
                } else if *i < 4 {
                    2
                } else if *i < 5 {
                    3
                } else {
                    4
                },
                0,
            ),
            Pos::Room(r, i) => ((*r + 1) * 2, i + 1),
        }
    }

    fn dist(&self, other: &Self) -> usize {
        let (x1, y1) = self.as_coords();
        let (x2, y2) = other.as_coords();
        if y1 > 0 && y2 > 0 {
            euclidean_dist(x1, y1, x1, 0) + euclidean_dist(x1, 0, x2, y2)
        } else {
            euclidean_dist(x1, y1, x2, y2)
        }
    }

    fn cost(&self, other: &Self, k: &Amphipod) -> usize {
        let d = self.dist(other);
        let r = d * match k {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
            No => panic!("Cant calc movement cost for no Amphipod"),
        };
        // println!("{:?} {} -> {}: {}", k, self, other, r);
        r
    }
}

fn euclidean_dist(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_diff = x1.max(x2) - x1.min(x2);
    let y_diff = y1.max(y2) - y1.min(y2);
    x_diff + y_diff
}

fn possible_postions(p: &Positions, pos: Pos) -> Vec<Pos> {
    let mut res = Vec::new();
    // println!("PossiblePos for {}", pos);
    match pos {
        Pos::Storage(pos) => {
            let mut room_limit_end = 4;
            for s_end in pos + 1..p.storage.len() {
                if p.storage[s_end] != No {
                    room_limit_end = room_limit_end.min(s_end - 1);
                    break;
                }
                res.push(Pos::Storage(s_end))
            }
            let mut room_limit_start = 0;
            for s_start in (0..pos).rev() {
                if p.storage[s_start] != No {
                    room_limit_start = room_limit_start.max(s_start);
                    break;
                }
                res.push(Pos::Storage(s_start))
            }
            let idx = p.storage[pos].idx().unwrap();
            if idx < room_limit_end && idx >= room_limit_start {
                for i in 0..p.rooms[idx].len() {
                    if p.rooms[idx][i] != No {
                        break;
                    }
                    res.push(Pos::Room(idx, i))
                }
            }
        }
        Pos::Room(r, i) => {
            if p.rooms[r][i..] != FINAL_POS.rooms[r][i..]
                && p.rooms[r][..i].iter().all(|x| x == &No)
            {
                let mut room_limit_end = 4;
                for s_end in r + 2..p.storage.len() {
                    if p.storage[s_end] != No {
                        room_limit_end = room_limit_end.min(s_end - 1);
                        break;
                    }
                    res.push(Pos::Storage(s_end))
                }
                // println!("RL end {}", room_limit_end);
                let mut room_limit_start = 0;
                for s_start in (0..(r + 2)).rev() {
                    if p.storage[s_start] != No {
                        room_limit_start = room_limit_start.max(s_start);
                        break;
                    }
                    res.push(Pos::Storage(s_start))
                }
                // println!("RL start {}", room_limit_start);
                let idx = p.rooms[r][i].idx().unwrap();
                if idx < room_limit_end && idx >= room_limit_start {
                    for i in 0..p.rooms[idx].len() {
                        if p.rooms[idx][i] != No {
                            break;
                        }
                        res.push(Pos::Room(idx, i))
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let lines = include_str!("../../input.txt")
        .lines()
        .skip(2)
        .collect::<Vec<_>>();
    let mut p = Positions::default();
    for (i, ci) in [3, 5, 7, 9].into_iter().enumerate() {
        for (j, line) in lines.iter().enumerate().take(p.rooms[0].len()) {
            println!("{} {}", i, j);
            p.rooms[i][j] = line[ci..=ci].chars().next().unwrap().into();
        }
    }
    // let p = Positions { storage: [No, No, No,No, A,D,B], rooms: [[B,A], [No,D], [C,C], [No,No]] };
    println!("{}", p);
    // for (p, cost) in successors(&p) {
    //     println!("COST: {}", cost);
    //     println!("{}", p)
    // };
    let (p, cost) =
        pathfinding::directed::dijkstra::dijkstra(&(p, 0), successors, |(p, _)| p == &FINAL_POS)
            .unwrap();
    for (i, (p, cost)) in p.iter().enumerate() {
        println!("I {}  COST: {}", i, cost);
        println!("{}", p)
    }
    println!(">>> {}", cost);
}
