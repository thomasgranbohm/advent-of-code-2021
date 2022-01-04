fn main() {
	let mut s = String::new();
	let line = include_str!("../input.txt").to_string();
	let mut nums: Vec<i32> = Vec::new();

	for c in line.chars() {
		if c == ',' && s.len() > 0 {
			nums.push(s.parse::<i32>().unwrap());
			s = String::new();
		} else {
			s.push(c);
		}
	}
	nums.push(s.parse::<i32>().unwrap());
	nums.sort();

	part1(&nums);
	part2(&nums);
}

fn factorial(num: i64) -> i64 {
	num * (num + 1) / 2
}

fn part1(nums: &Vec<i32>) {
	let mid = nums[nums.len() / 2];

	let mut total_fuel = 0;
	for n in nums {
		total_fuel += (n - mid).abs();
	}

	println!("Part One: {}", total_fuel);
}

fn part2(nums: &Vec<i32>) {
	let mut max = -1;
	for &n in nums {
		if max < n || max == -1 {
			max = n
		}
	}

	let facts: Vec<i64> = nums.iter().map(|a| factorial(*a as i64)).collect();
	let mut lower = -1;
	for a in 0..(max + 1) {
		let mut total_fuel = 0;

		for (i, _) in facts.iter().enumerate() {
			total_fuel += factorial(((nums[i] - a) as i64).abs());
		}

		if total_fuel < lower || lower == -1 {
			lower = total_fuel;
		}
	}

	println!("Part Two: {}", lower);
}
