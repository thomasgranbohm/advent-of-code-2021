fn main() {
	let lines = include_str!("../input.txt")
		.lines()
		.map(|a| a.to_string())
		.collect();

	part1(&lines);
}

fn char_to_int(c: char) -> i16 {
	match c {
		'(' => 3,
		'[' => 57,
		'{' => 1197,
		'<' => 25137,
		')' => -3,
		']' => -57,
		'}' => -1197,
		'>' => -25137,
		_ => 0,
	}
}

fn part1(lines: &Vec<String>) {
	let mut illegal: i32 = 0;
	for line in lines {
		let mut q: Vec<i16> = Vec::new();
		for c in line.chars() {
			let cv = char_to_int(c);
			if cv > 0 {
				q.push(cv);
			} else if cv < 0 {
				let top = q.pop().unwrap();
				if top != -cv {
					illegal += -cv as i32;
					break;
				}
			}
		}
	}

	println!("Part One: {}", illegal);
}
