use std::collections::HashMap;

const DIE_DISTR: [u8; 7] = [1, 3, 6, 7, 6, 3, 1]; // Index 0 is result 3

const P1_TURN: bool = true;

fn main() {
    let mut starting_pos = include_str!("../../input.txt")
        .lines()
        .map(|x| x.trim()[28..].parse::<u8>().unwrap());
    let player1_pos = starting_pos.next().unwrap();
    let player2_pos = starting_pos.next().unwrap();
    let mut positions = HashMap::new();
    positions.insert((player1_pos, player2_pos), vec![(0, 0, 1u64, P1_TURN)]);
    let mut p1_winning: u64 = 0;
    let mut p2_winning: u64 = 0;
    // let mut i = 0;
    while !positions.is_empty() {
        let pos = positions.keys().next().copied().unwrap();
        let scores = positions.remove(&pos).unwrap();

        for (i, amt) in DIE_DISTR.iter().enumerate() {
            let movement = i + 3;
            let mut p1_mov = Vec::new();
            let mut p2_mov = Vec::new();
            let p1_pos = (((pos.0 as usize + movement) % 10) as u8, pos.1);
            let p2_pos = (pos.0, ((pos.1 as usize + movement) % 10) as u8);
            // println!("MOV: {}, P1: {:?}, P2: {:?}", movement, p1_pos, p2_pos);
            for (p1_score, p2_score, count, turn) in &scores {
                if *turn == P1_TURN {
                    let score = p1_score + if p1_pos.0 == 0 { 10 } else { p1_pos.0 };
                    // println!("P1: {} {}", score, count * (*amt as u64));
                    if score >= 21 {
                        p1_winning += count * (*amt as u64);
                    } else {
                        p1_mov.push((score, *p2_score, count * (*amt as u64), !turn))
                    }
                } else {
                    let score = p2_score + if p2_pos.1 == 0 { 10 } else { p2_pos.1 };
                    // println!("P2: {} {}", score, count * (*amt as u64));
                    if score >= 21 {
                        p2_winning += count * (*amt as u64);
                    } else {
                        p2_mov.push((*p1_score, score, count * (*amt as u64), !turn))
                    }
                }
            }
            if !p1_mov.is_empty() {
                positions.entry(p1_pos).and_modify(|x| x.extend(p1_mov.iter().copied())).or_insert(p1_mov);
            }

            if !p2_mov.is_empty() {
                positions.entry(p2_pos).and_modify(|x| x.extend(p2_mov.iter().copied())).or_insert(p2_mov);
            }
        }
        // i+=1;
        // println!("{} {:?}",i,positions);
        // if i == 5 {
        //     break;
        // }
    }
    println!("P1: {}", p1_winning);
    println!("P2: {}", p2_winning);
}
