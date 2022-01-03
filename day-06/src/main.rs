fn main() {
	let fishes = include_str!("../input.txt")
		.to_string()
		.chars()
		.filter(|c| c != &',')
		.map(|c| c.to_string().parse::<i64>().unwrap())
		.collect();

	part1(&fishes);
}

fn calc_fishes(fishes: &Vec<i64>, days: i64) -> i64 {
	let mut a: Vec<i64> = vec![0; days as usize];
	let mut total_fishes = fishes.len() as i64;

	for fish in fishes.iter() {
		a[*fish as usize] += 1;
	}

	for current_day in 0..days {
		let f = a[current_day as usize];
		total_fishes += f;
		if current_day + 7 < days {
			a[(current_day + 7) as usize] += f;
		}
		if current_day + 9 < days {
			a[(current_day + 9) as usize] += f;
		}
	}

	total_fishes
}

fn part1(fishes: &Vec<i64>) {
	println!("Part One: {}", calc_fishes(&fishes, 80));
}
