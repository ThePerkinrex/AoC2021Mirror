use std::collections::HashMap;

fn possible_paths<'a>(
    node: &'a str,
    connections: &'a HashMap<&str, Vec<&str>>,
    mut visited: Vec<&'a str>,
    can_visit_double: bool,
) -> Vec<Vec<&'a str>> {
    if node == "end" {
        return vec![vec!["end"]];
    }
    visited.push(node);
    let i = connections[node]
        .iter()
        .filter(|x| !visited.contains(x) || x.to_uppercase() == x.to_string())
        .flat_map(|x| {
            // println!("{} {:?}", x, visited);
            possible_paths(*x, connections, visited.clone(), can_visit_double)
                .into_iter()
                .map(|mut x| {
                    x.insert(0, node);
                    // println!(">>> {:?}", x);
                    x
                })
        });
    if can_visit_double {
        i.chain(
            connections[node]
                .iter()
                .filter(|x| {
                    visited.contains(x) && x.to_uppercase() != x.to_string() && *x != &"start"
                })
                .flat_map(|x| {
                    // println!("{} {:?}", x, visited);
                    possible_paths(*x, connections, visited.clone(), false)
                        .into_iter()
                        .map(|mut x| {
                            x.insert(0, node);
                            // println!(">>> {:?}", x);
                            x
                        })
                }),
        )
        .collect()
    } else {
        i.collect()
    }
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

    let p = possible_paths("start", &connections, Vec::new(), true);
    // p.sort_by(|a, b| a.join(",").partial_cmp(&b.join(",")).unwrap());
    // for l in &p {
    //     println!("{}", l.join(","))
    // }
    println!(">>> {}", p.len())
}
