fn main() {
	let lines: Vec<Vec<String>> = include_str!("../input.txt")
		.lines()
		.map(|a| parse_line(a.to_string()))
		.collect();

	part1(&lines);
}

fn parse_line(line: String) -> Vec<String> {
	let mut a = String::new();
	let mut b: Vec<String> = Vec::new();

	let mut past_delimiter = false;

	for c in line.chars() {
		if past_delimiter {
			if c == ' ' && a.len() > 0 {
				b.push(a);
				a = String::new();
			} else if c != '|' && c != ' ' {
				a.push(c);
			}
		} else if c == '|' {
			past_delimiter = true;
		}
	}
	b.push(a);

	b
}

fn part1(lines: &Vec<Vec<String>>) {
	let mut uses = 0;

	for line in lines {
		for letter in line {
			if letter.len() == 2 || letter.len() == 3 || letter.len() == 4 || letter.len() == 7 {
				uses += 1;
			}
		}
	}

	println!("Part One: {} uses", uses);
}
