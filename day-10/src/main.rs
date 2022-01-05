fn main() {
	let lines = include_str!("../input.txt")
		.lines()
		.map(|a| a.to_string())
		.collect();

	part1(&lines);
	part2(&lines);
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

fn int_to_int(i: i16) -> i16 {
	match i {
		3 => 1,
		57 => 2,
		1197 => 3,
		25137 => 4,
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

fn part2(lines: &Vec<String>) {
	let mut illegals: Vec<i64> = Vec::new();

	for line in lines {
		let mut q: Vec<i16> = Vec::new();
		let mut has_illegal = false;

		for c in line.chars() {
			let cv = char_to_int(c);
			if cv > 0 {
				q.push(cv);
			} else if cv < 0 {
				let top = q.pop().unwrap();
				if top != -cv {
					q.push(top);
					has_illegal = true;
					break;
				}
			}
		}

		if !has_illegal && q.len() > 0 {
			let mut i: i64 = 0;
			q.reverse();
			for n in q {
				i *= 5;
				i += int_to_int(n) as i64;
			}
			illegals.push(i);
		}
	}

	illegals.sort();

	println!("Part Two: {:?}", illegals[illegals.len() / 2]);
}
