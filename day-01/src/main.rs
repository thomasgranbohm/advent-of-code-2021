use std::io::{self, BufRead};

fn main() {
	let lines: Vec<i32> = io::stdin()
		.lock()
		.lines()
		.map(|value| value.expect("Could not parse value"))
		.map(|value| value.parse::<i32>().unwrap())
		.collect();

	part1(lines);
}

fn part1(lines: Vec<i32>) {
	let mut previous = -1;
	let mut increased = 0;

	for num in lines {
		if previous != -1 {
			if num - previous > 0 {
				increased += 1;
			}
		}
		previous = num;
	}

	println!("Part One: {}", increased);
}
