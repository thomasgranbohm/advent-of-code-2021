use std::io::{self, BufRead};

fn main() {
	let lines: Vec<String> = io::stdin()
		.lock()
		.lines()
		.map(|value| value.expect("Could not parse value"))
		.collect();

	part1(&lines);
}

fn part1(lines: &Vec<String>) {
	let mut horizontal = 0;
	let mut vertical = 0;

	for index in 0..lines.len() {
		let line = &lines[index].to_string();

		if line.starts_with("forward ") {
			let movement = line.replace("forward ", "").parse::<i32>().unwrap();
			horizontal += movement;
		} else if line.starts_with("down ") {
			let movement = line.replace("down ", "").parse::<i32>().unwrap();
			vertical += movement;
		} else if line.starts_with("up ") {
			let movement = line.replace("up ", "").parse::<i32>().unwrap();
			vertical -= movement;
		}
	}

	println!("Part One: {}", horizontal * vertical);
}
