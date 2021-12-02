fn main() {
    let mut f = 0;
    let mut d = 0;
    for l in include_str!("../../input.txt").lines() {
        if l.starts_with("up") {
            // println!("U {:?}", &l[3..]);
            d -= l[3..].parse::<isize>().unwrap();
        }else if l.starts_with("down") {
            // println!("D {:?}", &l[5..]);
            d += l[5..].parse::<isize>().unwrap();
        }else{
            // println!("F {:?}", &l[8..]);
            f += l[8..].parse::<isize>().unwrap();
        }
    }

    println!(">>> {}", f*d);
}
