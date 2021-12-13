fn main() {
    let sum = include_str!("../../input.txt")
        .lines()
        .fold(0, |count, line| {
            let mut stack = vec![];
            let mut last = None;
            for c in line.chars() {
                if let '(' | '[' | '{' | '<' = c {
                    stack.push(c)
                } else if !matches!(
                    (stack.pop().unwrap(), c),
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
                ) {
                    last = Some(c);
                    break;
                }
            }
            // println!("L {} :: {:?}", line, last);
            count
                + match last {
                    Some(')') => 3,
                    Some(']') => 57,
                    Some('}') => 1197,
                    Some('>') => 25137,
                    _ => 0,
                }
        });

    println!(">> {}", sum);
}
