fn main() {
    let r = include_str!("../../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .fold((None, 0), |(last, sum), n| {
            (
                Some(n),
                if last.map(|x| x < n).unwrap_or(false) {
                    1
                } else {
                    0
                } + sum,
            )
        })
        .1;

    println!(">> {}", r);
}
