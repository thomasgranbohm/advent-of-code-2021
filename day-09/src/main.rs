fn main() {
	let numbers: Vec<Vec<i32>> = include_str!("../input.txt")
		.lines()
		.map(|a| {
			let mut nums: Vec<i32> = Vec::new();

			for c in a.chars() {
				nums.push(c.to_string().parse::<i32>().unwrap())
			}

			nums
		})
		.collect();

	part1(&numbers);
}

fn part1(numbers: &Vec<Vec<i32>>) {
	let mut lowpoints: Vec<(usize, usize)> = Vec::new();

	for (ai, a) in numbers.iter().enumerate() {
		for (bi, b) in a.iter().enumerate() {
			let mut is_lowpoint = true;

			if ai as i32 - 1 >= 0 {
				if is_lowpoint {
					is_lowpoint = numbers[ai - 1][bi] > *b;
				}
			}
			if ai + 1 < numbers.len() {
				if is_lowpoint {
					is_lowpoint = numbers[ai + 1][bi] > *b;
				}
			}
			if bi as i32 - 1 >= 0 {
				if is_lowpoint {
					is_lowpoint = numbers[ai][bi - 1] > *b;
				}
			}
			if bi + 1 < a.len() {
				if is_lowpoint {
					is_lowpoint = numbers[ai][bi + 1] > *b;
				}
			}

			if is_lowpoint {
				lowpoints.push((bi, ai));
			}
		}
	}

	let sum: i32 = lowpoints.iter().map(|a| (numbers[a.1][a.0] + 1)).sum();

	println!("Part One: {}", sum);
}
