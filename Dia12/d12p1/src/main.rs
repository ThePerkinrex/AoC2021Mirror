use std::{collections::HashMap};

fn possible_paths<'a>(
    node: &'a str,
    connections: &'a HashMap<&str, Vec<&str>>,
    mut visited: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    if node == "end" {
        return vec![vec!["end"]];
    }
    visited.push(node);
    connections[node]
        .iter()
        .filter(|x| !visited.contains(x) || x.to_uppercase() == x.to_string())
        .flat_map(|x| {
            // println!("{} {:?}", x, visited);
            possible_paths(*x, connections, visited.clone())
                .into_iter()
                .map(|mut x| {
                    x.insert(0, node);
                    // println!(">>> {:?}", x);
                    x
                })
        }).collect()
}

fn main() {
    let mut connections = HashMap::new();

    for line in include_str!("../../input.txt")
        .lines()
        .map(|x| x.split('-').collect::<Vec<&str>>())
    {
        connections
            .entry(line[0])
            .or_insert_with(Vec::new)
            .push(line[1]);
        connections
            .entry(line[1])
            .or_insert_with(Vec::new)
            .push(line[0]);
    }

    let p = possible_paths("start", &connections, Vec::new());
    println!(">>> {}", p.len())
}
