fn main() {
    let lines: Vec<_> = include_str!("../../input.txt").lines().collect();

    let mut gamma = vec!["0"; lines[0].len()];
    let mut epsilon = vec!["1"; gamma.len()];
    let mut count = 0;
    let mut counts = Vec::new();
    for i in 0..gamma.len() {
        for line in &lines {
            if line.chars().nth(i).unwrap() == '1' {
                count += 1;
                if count >= lines.len() / 2 {
                    gamma[i] = "1";
                    epsilon[i] = "0";
                }
            }
        }
        counts.push(count);
        // console.log(gamma)
        count = 0;
    }
    let mut co2 = lines.clone();
    let mut co2_counts = counts.clone();
    let mut o2 = lines.clone();
    let mut o2_counts = counts.clone();
    for i in 0..=gamma.len() {
        // println!("{:?}", o2_counts);
        // println!("{:?}", co2_counts);
        let currlen = o2.len();
        let currcounts = o2_counts.clone();
        for j in (0..o2.len()).rev() {
            if o2.len() == 1 {
                break;
            }
            if ((currcounts[i] as f64) >= (currlen as f64) / 2.
                && o2[j].chars().nth(i).unwrap() == '0')
                || ((currcounts[i] as f64) < (currlen as f64) / 2.
                    && o2[j].chars().nth(i).unwrap() == '1')
            {
                for (x, c) in o2_counts.iter_mut().enumerate().take(o2[j].len()) {
                    if o2[j].chars().nth(x).unwrap() == '1' {
                        // println!(">> {}", x);
                        *c -= 1
                    }
                }

                o2.remove(j);
            }
        }

        let currlen = co2.len();
        let currcounts = co2_counts.clone();
        for j in (0..co2.len()).rev() {
            if co2.len() == 1 {
                break;
            }
            if ((currcounts[i] as f64) >= (currlen as f64) / 2.
                && co2[j].chars().nth(i).unwrap() == '1')
                || ((currcounts[i] as f64) < (currlen as f64) / 2.
                    && co2[j].chars().nth(i).unwrap() == '0')
            {
                for (x, c) in co2_counts.iter_mut().enumerate().take(co2[j].len()) {
                    if co2[j].chars().nth(x).unwrap() == '1' {
                        *c -= 1
                    }
                }
                co2.remove(j);
            }
        }
    }
    println!(" o2: {}", o2[0]);
    println!("co2: {}", co2[0]);
    let o2 = usize::from_str_radix(o2[0], 2).unwrap();
    let co2 = usize::from_str_radix(co2[0], 2).unwrap();
    println!(" o2: {}", o2);
    println!("co2: {}", co2);
    println!(">>> {}", o2 * co2);
}
