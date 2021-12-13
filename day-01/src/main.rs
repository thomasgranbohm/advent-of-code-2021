use std::io::{self, BufRead};

fn main() {
	let lines: Vec<i32> = io::stdin()
		.lock()
		.lines()
		.map(|value| value.expect("Could not parse value"))
		.map(|value| value.parse::<i32>().unwrap())
		.collect();

	part1(&lines);
	part2(&lines);
}

fn part1(lines: &Vec<i32>) {
	let mut previous = -1;
	let mut increased = 0;

	for num in lines {
		if previous != -1 {
			if num - previous > 0 {
				increased += 1;
			}
		}
		previous = *num;
	}

	println!("Part One: {}", increased);
}

fn part2(lines: &Vec<i32>) {
	let mut previous = -1;
	let mut increased = 0;

	for index in 2..lines.len() {
		let sum = lines[index - 2] + lines[index - 1] + lines[index];
		if previous != -1 && previous < sum {
			increased += 1;
		}

		previous = sum;
	}

	println!("Part Two: {}", increased);
}
