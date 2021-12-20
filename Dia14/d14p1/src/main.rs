use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../../input.txt").lines();
    let mut template = lines.next().unwrap().to_string();
    lines.next().unwrap();
    let mut rules = Vec::new();
    lines.for_each(|x| {
        let mut s = x.split(" -> ");
        let (a, b) = (s.next().unwrap(), s.next().unwrap());
        rules.push((a, b));
    });
    // println!("{}", template);

    for _ in 0..10 {
        let mut new_text = String::new();
        new_text += &template[..1];
        for i in 0..(template.len() - 1) {
            let s = &template[i..(i + 2)];
            // println!(">> {}", s);
            for (cond, c) in &rules {
                if *cond == s {
                    // println!(">>>>> {} -> {}", cond, c);
                    new_text += c;
                    break;
                }
            }
            new_text += &s[1..];
        }
        template = new_text;
    }
    let counts = template.chars().fold(HashMap::new(), |mut counts, c| {
        counts.entry(c).and_modify(|x| *x += 1).or_insert(1);
        counts
    });

    let (min, max) = (
        counts.values().min().unwrap(),
        counts.values().max().unwrap(),
    );
    println!(">>> {}", max - min);
}
