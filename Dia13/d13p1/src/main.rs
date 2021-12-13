use std::str::FromStr;

#[derive(Debug)]
enum Axis {
    X(usize),
    Y(usize)
}

impl FromStr for Axis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[..1] == "x" {
            Ok(Self::X(s[2..].parse().unwrap()))
        }else{
            Ok(Self::Y(s[2..].parse().unwrap()))
        }
    }
}

impl Axis {
    fn apply(&self, map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        match self {
            Axis::X(x) => {
                let mut res = vec![vec![0; *x]; map.len()];
                for y in 0..map.len() {
                    for x in 0..*x {
                        res[y][x] = map[y][x]
                    }
                    for (x_res, x_old) in (0..(map[y].len()-(*x+1))).map(|i| (*x-i-1, *x+i+1)) {
                        if map[y][x_old] == 1 {
                            res[y][x_res] = map[y][x_old]

                        }
                    }
                }
                
                res
            },
            Axis::Y(y) => {
                let mut res = vec![vec![0; map[0].len()]; *y];
                for x in 0..map[0].len() {
                    for y in 0..*y {
                        res[y][x] = map[y][x]
                    }
                    for (y_res, y_old) in (0..(map.len()-(*y+1))).map(|i| (*y-i-1, *y+i+1)) {
                        if map[y_old][x] == 1 {
                            res[y_res][x] = map[y_old][x]
                        }
                    }
                }
                
                res
            }
            ,
        }
    }
}

fn main() {
    let mut file = include_str!("../../input.txt").split("\n\n");
    let dots = file.next().unwrap().lines().map::<(usize, usize), _>(|x| {
        let mut s = x.split(',');
        (s.next().unwrap().parse().map_err(|e| {println!("{:?}", x); e}).unwrap(), s.next().unwrap().parse().unwrap())
    }).collect::<Vec<_>>();
    let(x_max, y_max) = dots.iter().fold((0, 0), |(x_max, y_max), (x,y)|(x_max.max(*x), y_max.max(*y)));
    
    let mut map = dots.into_iter().map(|(x, y)| (x, y)).fold(vec![vec![0; x_max+1]; y_max+1], |mut map, (x,y)| {
        map[y][x] = 1;
        map
    });
    
    let mut instructions = file.next().unwrap().lines().map::<Axis, _>(|x| x[11..].parse().unwrap());
    map = instructions.next().unwrap().apply(map);
    let s: usize = map.iter().map(|x| x.iter().filter(|x| **x == 1).count()).sum();
    println!(">>> {}", s);
}
