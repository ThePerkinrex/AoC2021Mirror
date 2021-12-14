// use std::collections::HashMap;

// fn apply_rules(rules: &[(char, char, char)], right: char, left: char, max_depth: usize) -> HashMap<char, usize> {
//     println!(">>{1:>>0$} {0}", max_depth, "");
//     let mut res = HashMap::new();
//     if max_depth != 0 {
        
//         for (r, l, c) in rules {
//             if &right == r && &left == l {
//                 let c = *c;
//                 res.insert(c, 1);
//                 apply_rules(rules, right, c, max_depth-1).into_iter().for_each(|(k,v)| {res.entry(k).and_modify(|x| *x += v).or_insert(v);});
//                 apply_rules(rules, c, left, max_depth-1).into_iter().for_each(|(k,v)| {res.entry(k).and_modify(|x| *x += v).or_insert(v);});
                
//                 break;
//             }
//         }
//     }
//     println!("<<{1:<>0$} {0}", max_depth, "");
//     res
// }

// fn main() {
//     let mut lines = include_str!("../../input2.txt").lines();
//     let mut template = lines.next().unwrap();
//     lines.next().unwrap();
//     let mut rules = Vec::new();
//     lines.for_each(|x| {
//         let mut s = x.split(" -> ");
//         let mut a = s.next().unwrap().chars();
//         let x = (a.next().unwrap(), a.next().unwrap(), s.next().unwrap().chars().next().unwrap());
//         rules.push(x);
//     });
    
//     let mut counts = HashMap::new();
//     let chars: Vec<_> = template.chars().collect();
//     for i in 0..(chars.len()-1) {
//         apply_rules(&rules, chars[i], chars[i+1], 40).into_iter().for_each(|(k,v)| {counts.entry(k).and_modify(|x| *x += v).or_insert(v);});
//         println!("{}", i);
//     }

//     let (min, max) = (counts.values().min().unwrap(), counts.values().max().unwrap());
//     println!(">>> {}", max- min);
// }

use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../../input.txt").lines();
    let template = lines.next().unwrap().to_string();
    lines.next().unwrap();
    let mut rules = Vec::new();
    lines.for_each(|x| {
        let mut s = x.split(" -> ");
        let a = s.next().unwrap().to_string();
        let ch = s.next().unwrap().chars().next().unwrap();
        let b = format!("{}{}", &a[..1], ch);
        let c = format!("{}{}", ch, &a[1..]);
        rules.push((a, b, c, ch));
    });

    let mut adj = HashMap::new();
    let mut counts = HashMap::new();
    for i in 0..(template.len()-1) {
        adj.entry(template[i..(i+2)].to_string()).and_modify(|x| *x+=1).or_insert(1u64);
        counts.entry(template[i..(i+1)].chars().next().unwrap()).and_modify(|x| *x+=1).or_insert(1);
    }
    
    counts.entry(template[(template.len()-1)..].chars().next().unwrap()).and_modify(|x| *x+=1).or_insert(1);

    for _ in 0..40 {
        let mut new_adj = HashMap::new();
        // println!("{:?}", counts);
        // println!("{:?}", adj);
        for (k, v) in adj.into_iter() {
            for (ini, res1, res2, ch) in &rules {
                if &k == ini {
                    // println!("{} {} {}", res1, res2, v);
                    new_adj.entry(res1.clone()).and_modify(|x| *x+=v).or_insert(v);
                    new_adj.entry(res2.clone()).and_modify(|x| *x+=v).or_insert(v);
                    counts.entry(*ch).and_modify(|x| *x+=v).or_insert(v);
                    break;
                }
            }
        }
        adj = new_adj;
    }
    

    let (min, max) = (counts.values().min().unwrap(), counts.values().max().unwrap());
    // println!("{} {}", min, max);
    println!(">>> {}", max- min);
}
