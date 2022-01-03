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
}
fn part1(nums: &Vec<i32>) {
	let mid = nums[nums.len() / 2];

	let mut total_fuel = 0;
	for n in nums {
		total_fuel += (n - mid).abs();
	}

	println!("Part One: {}", total_fuel);
}
