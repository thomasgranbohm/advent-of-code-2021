use std::cmp;

fn main() {
	let mut max_x = 0;
	let mut max_y = 0;

	let n_c = [',', ' ', '-', '>'];
	let lines: Vec<[i32; 4]> = include_str!("../input.txt")
		.lines()
		.map(|value| {
			let x: String = value.parse().unwrap();
			let mut line: [i32; 4] = [0; 4];

			let mut n = String::new();
			let mut i = 0;
			for c in x.chars() {
				if n_c.contains(&c) && n.len() > 0 {
					line[i] = n.parse::<i32>().unwrap();
					i += 1;
					n = String::new();
				} else if !n_c.contains(&c) {
					n.push_str(&String::from(c));
				}
			}
			line[i] = n.parse::<i32>().unwrap();

			let local_x = cmp::max(line[0], line[2]);
			let local_y = cmp::max(line[1], line[3]);

			max_x = cmp::max(local_x, max_x);
			max_y = cmp::max(local_y, max_y);

			if line[0] > line[2] || line[1] > line[3] {
				line = [line[2], line[3], line[0], line[1]];
			}

			line
		})
		.collect();

	part1(&lines, max_x, max_y);
	part2(&lines, max_x, max_y);
}

fn part1(lines: &Vec<[i32; 4]>, max_x: i32, max_y: i32) {
	let mut overlaps = 0;
	let mut grid = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

	for l in lines.iter() {
		if l[0] == l[2] {
			for y in l[1]..(l[3] + 1) {
				grid[y as usize][l[0] as usize] += 1;
			}
		} else if l[1] == l[3] {
			for x in l[0]..(l[2] + 1) {
				grid[l[1] as usize][x as usize] += 1;
			}
		}
	}

	for y in 0..(max_y + 1) {
		for x in 0..(max_x + 1) {
			overlaps += if grid[y as usize][x as usize] > 1 {
				1
			} else {
				0
			};
		}
	}

	println!("Part One: {} matches", overlaps);
}

fn part2(lines: &Vec<[i32; 4]>, max_x: i32, max_y: i32) {
	let mut overlaps = 0;
	let mut grid = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

	for l in lines.iter() {
		if l[0] == l[2] {
			for y in l[1]..(l[3] + 1) {
				grid[y as usize][l[0] as usize] += 1;
			}
		} else if l[1] == l[3] {
			for x in l[0]..(l[2] + 1) {
				grid[l[1] as usize][x as usize] += 1;
			}
		} else if l[1] < l[3] {
			let mut x = l[0];
			for y in l[1]..(l[3] + 1) {
				grid[y as usize][x as usize] += 1;
				x += if l[0] > l[2] { -1 } else { 1 };
			}
		} else if l[0] < l[2] {
			let mut x = l[2];
			for y in l[3]..(l[1] + 1) {
				grid[y as usize][x as usize] += 1;
				x += if l[2] > l[0] { -1 } else { 1 };
			}
		}
	}

	for y in 0..(max_y + 1) {
		for x in 0..(max_x + 1) {
			overlaps += if grid[y as usize][x as usize] > 1 {
				1
			} else {
				0
			};
		}
	}

	println!("Part Two: {} matches", overlaps);
}
