fn main() {
	let steps = 100;

	let mut grid: Vec<Vec<i8>> = include_str!("../input.txt")
		.lines()
		.map(|a| {
			let line = a.to_string();

			let mut v: Vec<i8> = Vec::new();

			for c in line.chars() {
				v.push(c.to_string().parse::<i8>().unwrap());
			}

			v
		})
		.collect();

	let max_y = grid.len();
	let max_x = grid[0].len();

	let positions = vec![
		(-1, 1),
		(-1, 0),
		(-1, -1),
		(0, -1),
		(1, -1),
		(1, 0),
		(1, 1),
		(0, 1),
	];

	let mut flashes = 0;
	let mut synced_step = -1;
	let mut step = 0;

	while synced_step == -1 {
		let mut new_flashes = 0;
		let mut flashed: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
		let mut q: Vec<(usize, usize)> = Vec::new();

		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				grid[y][x] += 1;
				if grid[y][x] > 9 {
					q.push((x, y))
				}
			}
		}

		while q.len() > 0 {
			let p = q.pop().unwrap();
			let x = p.0;
			let y = p.1;

			if grid[y][x] > 9 {
				grid[y][x] = 0;
				flashed[y][x] = true;

				new_flashes += 1;

				for pos in positions.iter() {
					let nx = (x as i8 + pos.0) as usize;
					let ny = (y as i8 + pos.1) as usize;

					let in_range = ny as i8 >= 0 && ny < max_y && nx as i8 >= 0 && nx < max_x;

					if !in_range {
						continue;
					}

					if in_range && !flashed[ny][nx] {
						grid[ny][nx] += 1;
						q.push((nx, ny));
					}
				}
			}
		}

		if step < steps {
			flashes += new_flashes;
		}

		if new_flashes == max_x * max_y && synced_step == -1 {
			synced_step = step;
		}

		step += 1;
	}

	println!("Part One: {} flashes", flashes);
	println!("Part Two: {}", synced_step + 1);
}
