use std::{ops::Add, str::FromStr};

#[derive(Debug)]
enum Pair {
    Regular(usize),
    Pair(Box<Pair>, Box<Pair>),
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut left = "";
            let mut right = "";
            let mut count = 0;
            for i in 1..(s.len() - 1) {
                if &s[i..=i] == "[" {
                    count += 1;
                } else if &s[i..=i] == "]" {
                    count -= 1;
                }
                if count == 0 && &s[i..=i] == "," {
                    left = &s[1..i];
                    right = &s[(i + 1)..(s.len() - 1)];
                    break;
                }
            }
            let left = Pair::from_str(left)?;
            let right = Pair::from_str(right)?;
            Ok(Self::Pair(Box::new(left), Box::new(right)))
        } else {
            Ok(Self::Regular(s.parse().unwrap()))
        }
    }
}

impl Pair {
    fn reduce(self) -> Self {
        let mut s = self;
        // println!("REDUCE: {}", s);
        let mut applied = true;
        while applied {
            let (s1, b) = s.explode();
            s = s1;
            if b {
                // println!("Applied explode: {}", s);
                continue;
            }
            let (s1, b) = s.split();
            s = s1;
            if !b {
                applied = false;
            } else {

                // println!("Applied split: {}", s);
            }
        }
        s
    }

    fn explode(self) -> (Self, bool) {
        let (s, _, _, b) = self.explode_aux(0);
        (s, b)
    }

    fn explode_aux(self, depth: usize) -> (Self, usize, usize, bool) {
        if depth >= 4 {
            match self {
                Self::Pair(l, r) => match (l.as_ref(), r.as_ref()) {
                    (Self::Regular(l), Self::Regular(r)) => (Self::Regular(0), *l, *r, true),
                    _ => unreachable!("Unexpected pair of non regular numbers"),
                },
                _ => (self, 0, 0, false),
            }
        } else {
            match self {
                Pair::Pair(l, mut r) => {
                    let (mut l_exploded, ll, lr, has_exploded) = l.explode_aux(depth + 1);
                    if has_exploded {
                        r.add_to_leftmost(lr);
                        (Self::Pair(Box::new(l_exploded), r), ll, 0, true)
                    } else {
                        let (r_exploded, rl, rr, has_exploded) = r.explode_aux(depth + 1);
                        l_exploded.add_to_rightmost(rl);
                        (
                            Self::Pair(Box::new(l_exploded), Box::new(r_exploded)),
                            0,
                            rr,
                            has_exploded,
                        )
                    }
                }
                _ => (self, 0, 0, false),
            }
        }
    }

    fn add_to_leftmost(&mut self, n: usize) {
        match self {
            Pair::Regular(x) => *x += n,
            Pair::Pair(l, _) => l.add_to_leftmost(n),
        }
    }

    fn add_to_rightmost(&mut self, n: usize) {
        match self {
            Pair::Regular(x) => *x += n,
            Pair::Pair(_, r) => r.add_to_rightmost(n),
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Self::Regular(x) if x >= 10 => (
                Self::Pair(
                    Box::new(Self::Regular(x / 2)),
                    Box::new(Self::Regular((x as f64 / 2f64).ceil() as usize)),
                ),
                true,
            ),
            Self::Pair(l, r) => {
                let (l, b) = l.split();
                if b {
                    (Self::Pair(Box::new(l), r), b)
                } else {
                    let (r, b) = r.split();

                    (Self::Pair(Box::new(l), Box::new(r)), b)
                }
            }
            _ => (self, false),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Pair::Regular(x) => *x,
            Pair::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let s = Self::Pair(Box::new(self), Box::new(rhs));
        s.reduce()
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Regular(x) => write!(f, "{}", x),
            Pair::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

fn main() {
    // let file = r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    // [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    // [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    // [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    // [7,[5,[[3,8],[1,4]]]]
    // [[2,[2,2]],[8,[8,1]]]
    // [2,9]
    // [1,[[[9,3],9],[[9,0],[0,7]]]]
    // [[[5,[7,4]],7],1]
    // [[[[4,2],2],6],[8,7]]"#;
    let file = include_str!("../../input.txt");
    let mut pairs = file.lines().map(|x| x.trim().parse::<Pair>().unwrap());

    let mut last = pairs.next().unwrap();
    for p in pairs {
        // println!("::: {}", last);

        // println!(":::  + {}", p);

        last = last + p;
        // println!(":::  = {}", last);
        // println!();
    }
    // let last = Pair::from_str("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]").unwrap() + Pair::from_str("[[[[4,2],2],6],[8,7]]").unwrap();
    println!("{}", last);
    println!(">>> {}", last.magnitude());
}
