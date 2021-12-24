use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Var {
    X,Y,Z,W
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarOrLiteral {
    Var(Var),
    Lit(i64)
}

impl FromStr for Var {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            use Var::*;
            match &s[0..1] {
                "x" => Ok(X),
                "y" => Ok(Y),
                "z" => Ok(Z),
                "w" => Ok(W),
                _ => Err(())
            }
        }else{
            Err(())
        }
    }
}

impl Var {
    fn mut_ref<'a>(&self, x: &'a mut i64, y: &'a mut i64, z: &'a mut i64, w: &'a mut i64) -> &'a mut i64 {
        match self {
            Var::X => x,
            Var::Y => y,
            Var::Z => z,
            Var::W => w,
        }
    }
}

impl FromStr for VarOrLiteral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse() {
            Ok(Self::Lit(v))
        }else{
            Var::from_str(s).map(Self::Var)
        }
    }
}

impl VarOrLiteral {
    fn val(&self, mut x: i64, mut y: i64, mut z: i64, mut w: i64) -> i64 {
        match self {
            VarOrLiteral::Var(var) => *var.mut_ref(&mut x, &mut y, &mut z, &mut w),
            VarOrLiteral::Lit(x) => *x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instr {
    Inp(Var),
    Add(Var, VarOrLiteral),
    Mul(Var, VarOrLiteral),
    Div(Var, VarOrLiteral),
    Mod(Var, VarOrLiteral),
    Eql(Var, VarOrLiteral)
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        use Instr::*;
        match parts[0] {
            "inp" => Ok(Inp(parts[1].parse()?)),
            "add" => Ok(Add(parts[1].parse()?, parts[2].parse()?)),
            "mul" => Ok(Mul(parts[1].parse()?, parts[2].parse()?)),
            "div" => Ok(Div(parts[1].parse()?, parts[2].parse()?)),
            "mod" => Ok(Mod(parts[1].parse()?, parts[2].parse()?)),
            "eql" => Ok(Eql(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(())
        }
    }
}

impl Instr {
    fn apply<'a, I: Iterator<Item = &'a i64>>(&self, inp: &mut I, x: &mut i64, y: &mut i64, z: &mut i64, w: &mut i64) {
        match self {
            Instr::Inp(v) => {
                *v.mut_ref(x, y, z, w) = *inp.next().unwrap();
            },
            Instr::Add(a, b) => {
                *a.mut_ref(x, y, z, w) += b.val(*x, *y, *z, *w);
            },
            Instr::Mul(a, b) => {
                *a.mut_ref(x, y, z, w) *= b.val(*x, *y, *z, *w);
            },
            Instr::Div(a, b) => {
                *a.mut_ref(x, y, z, w) /= b.val(*x, *y, *z, *w);
            },
            Instr::Mod(a, b) => {
                *a.mut_ref(x, y, z, w) %= b.val(*x, *y, *z, *w);
            },
            Instr::Eql(a, b) => {
                *a.mut_ref(x, y, z, w) = if b.val(*x, *y, *z, *w) == *a.mut_ref(x, y, z, w) {1} else {0};
            },
        }
    }

    fn unwrap_lit(&self) -> i64 {
        match self {
            Instr::Add(_, VarOrLiteral::Lit(x)) => *x,
            Instr::Mul(_, VarOrLiteral::Lit(x)) => *x,
            Instr::Div(_, VarOrLiteral::Lit(x)) => *x,
            Instr::Mod(_, VarOrLiteral::Lit(x)) => *x,
            Instr::Eql(_, VarOrLiteral::Lit(x)) => *x,
            _ => panic!()
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct ALU {
    z: i64,
    digits_read: u8
}

// const PAIRS: [(i64, i64); 14] = [(13, 13), (11, 10), (15, 5), (-11, 14), (14, 5), (0, 15), (12, 4), (12, 11), (14, 1), (-6, 15), (-10, 12), (-12, 8), (-3, 14), (-5, 9)];



// fn run(alu: ALU, res: i64) -> Option<i64> {
//     if alu.digits_read == 14 {
//         if (alu.z == 0) {

//             println!("{}", res);
//         }
//         (alu.z == 0).then(||res)
//     }else{
//         (0..=9).rev().collect::<Vec<_>>().par_iter().map(|w| {
//             let w = *w;
//             let mut alu = alu;
//             let x = if ((alu.z % 26) + PAIRS[alu.digits_read as usize].0) != w {1} else {0};
//             let y = (26 * x) + 1;
//             // dbg!(alu.z, y);
//             alu.z = alu.z.overflowing_mul(y).0;
//             let y = (w + PAIRS[alu.digits_read as usize].1) * x;
//             alu.z += y;
//             let res = res + w * (10i64.pow((13-alu.digits_read) as u32));
//             alu.digits_read += 1;
//             run(alu, res)
//         }).find_first(|x| x.is_some()).map(Option::unwrap)
//     }
// }


fn solve(mut inp: [i64;14], cmds: Vec<Instr>) -> u64 {
    let mut stack = Vec::new();
    for i in 0..14 {
        let div = cmds[i*18+4].unwrap_lit();
        let chk = cmds[i*18+5].unwrap_lit();
        let add = cmds[i*18+15].unwrap_lit();
        if div == 1 {
            stack.push((i, add));
        }else {
            let (j, add) = stack.pop().unwrap();
            inp[i] = inp[j] + add + chk;
            if inp[i] > 9 {
                inp[j] -= inp[i] - 9;
                inp[i] = 9;
            }else if inp[i] < 1 {
                inp[j] += 1 - inp[i];
                inp[i] = 1;
            }
        }

    }
    inp.into_iter().rev().enumerate().fold(0, |r, (i, x)| r + ((x as u64) * 10u64.pow(i as u32)))
}

fn main() {
    let instr = include_str!("../../input.txt").lines().filter(|x| !x.trim().is_empty()).filter(|s| !s.starts_with('#')).map(|s| s.trim().parse::<Instr>().unwrap()).collect::<Vec<_>>();
    let r = solve([1;14], instr);
    println!(">>> {}", r)
    // let pairs = instr.split(|x| *x == Instr::Inp(Var::W)).filter(|x| !x.is_empty()).map(|x| (if let Instr::Add(Var::X, VarOrLiteral::Lit(v)) = x[4] {
    //     v
    // } else {unreachable!()}, if let Instr::Add(Var::Y, VarOrLiteral::Lit(v)) = x[14] {
    //     v
    // } else {unreachable!()})).collect::<Vec<_>>();
    
    // print!("[");
    // for (a,b) in pairs.iter() {
    //     print!("({}, {}), ", a, b)
    // }
    // println!("]")
    
}
