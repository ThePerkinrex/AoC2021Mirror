fn main() {
    let r = include_str!("../../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .fold((None, None, None, 0), |(last1, last2, last3, sum), n| {
            (
                last2,
                last3,
                Some(n),
                if last1
                    .zip(last2)
                    .zip(last3)
                    .map(|((a, b), c)| a + b + c < b + c + n)
                    .unwrap_or(false)
                {
                    1
                } else {
                    0
                } + sum,
            )
        })
        .3;

    println!(">> {}", r);
}
