fn main(){
	let lines: Vec<_> = include_str!("../../input.txt").lines().collect();

	let mut gamma = vec!["0"; lines[0].len()];
	let mut epsilon = vec!["1"; gamma.len()];
	let mut count = 0;


	for i in 0..gamma.len() {
		for line in &lines {
			if line.chars().nth(i).unwrap() == '1' {
				count += 1;
				if count >= lines.len() / 2 {
					gamma[i] = "1";
					epsilon[i] = "0";
					break
				}
			}
		}
		// console.log(gamma)
		count = 0;
	}
	let gamma = usize::from_str_radix(&gamma.join(""), 2).unwrap();
	let epsilon = usize::from_str_radix(&epsilon.join(""), 2).unwrap();
	println!("gamma: {}", gamma);
	println!("epsilon: {}", epsilon);
	println!(">>> {}", gamma * epsilon);
}