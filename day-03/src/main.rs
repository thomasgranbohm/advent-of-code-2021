fn main() {
	let lines: Vec<String> = include_str!("../input.txt")
		.lines()
		.map(|value| value.parse().unwrap())
		.collect();

	part1(&lines);
}

fn string_to_binary(num: &String) -> isize {
	isize::from_str_radix(&num, 2).unwrap()
}

fn part1(lines: &Vec<String>) {
	let mut columns: Vec<[i16; 2]> = Vec::new();
	for _ in 0..lines[0].len() {
		let column: [i16; 2] = [0, 0];
		columns.push(column);
	}

	for line in lines {
		let chars = line.chars();
		let mut index = 0;
		for c in chars {
			let num = c.to_digit(2).expect("Could not convert.");
			if num == 1 {
				columns[index][1] += 1;
			} else if num == 0 {
				columns[index][0] += 1;
			} else {
				continue;
			}
			index += 1;
		}
	}

	let mut num = String::new();

	for column in columns {
		if column[0] > column[1] {
			num.push_str("0");
		} else {
			num.push_str("1");
		}
	}

	let inverse_string = (0..num.len()).map(|_| '1').collect::<String>();
	let regular = string_to_binary(&num);
	let irregular = regular ^ string_to_binary(&inverse_string);

	println!("Part One: {}", regular * irregular);
}
