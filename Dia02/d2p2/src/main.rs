fn main() {
    let mut f = 0;
    let mut d = 0;
    let mut aim = 0;
    for l in include_str!("../../input.txt").lines() {
        if l.starts_with("up") {
            // println!("U {:?}", &l[3..]);
            aim -= l[3..].parse::<isize>().unwrap();
        }else if l.starts_with("down") {
            // println!("D {:?}", &l[5..]);
            aim += l[5..].parse::<isize>().unwrap();
        }else{
            // println!("F {:?}", &l[8..]);
            let x =  l[8..].parse::<isize>().unwrap();
            f += x;
            d += aim*x;
        }
    }

    println!(">>> {}", f*d);
}
