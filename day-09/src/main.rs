use std::cmp;

fn main() {
	let mut max_x = 0;
	let numbers: Vec<Vec<i32>> = include_str!("../input.txt")
		.lines()
		.map(|a| {
			let mut nums: Vec<i32> = Vec::new();

			for c in a.chars() {
				nums.push(c.to_string().parse::<i32>().unwrap())
			}

			max_x = cmp::max(max_x, nums.len());

			nums
		})
		.collect();
	let max_y = numbers.len();

	let lowpoints = part1(&numbers);
	part2(numbers, lowpoints, max_x, max_y);
}

fn part1(numbers: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
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

	lowpoints
}

fn part2(numbers: Vec<Vec<i32>>, lowpoints: Vec<(usize, usize)>, max_x: usize, max_y: usize) {
	let mut basins: Vec<i32> = Vec::new();

	let ns: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

	for point in lowpoints {
		let mut vs = vec![vec![false; max_x]; max_y];
		let mut to_check = vec![point];
		let mut basin_size = 0;

		while to_check.len() > 0 {
			let p = to_check.pop().unwrap();
			let x = p.0;
			let y = p.1;

			for n in ns {
				let new_x = (x as i8 + n.0) as usize;
				let new_y = (y as i8 + n.1) as usize;

				if new_y as i8 >= 0
					&& new_y < max_y && new_x as i8 >= 0
					&& new_x < max_x && vs[new_y][new_x] != true
					&& numbers[new_y][new_x] != 9
				{
					basin_size += 1;
					vs[new_y][new_x] = true;
					to_check.push((new_x, new_y));
				}
			}
		}

		basins.push(basin_size);
	}

	basins.sort();

	let mut sum = 1;
	for _ in 0..3 {
		sum *= basins.pop().unwrap();
	}

	println!("Part Two: {}", sum);
}
