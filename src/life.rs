use bevy::prelude::*;

#[derive(Resource)]
pub struct Conway {
	pub grid: Vec<Vec<LifeCell>>,
}

#[derive(Component, Clone, Hash)]
pub struct LifeCellRef {
	pub row: usize,
	pub col: usize,
	pub tick: usize,
}

pub struct LifeCell {
	pub entity: Option<Entity>,
	pub alive: bool,
}

impl Conway {
	pub fn new(size: u8) -> Self {
		let mut grid = Vec::with_capacity(size as usize);

		for _ in 0..size {
			let mut col = Vec::with_capacity(size as usize);
			for _ in 0..size {
				col.push(LifeCell {
					entity: None,
					alive: false, // if rng.gen_range(1..=6) > 3 { true } else { false },
				})
			}
			grid.push(col);
		}

		Self { grid }
	}

	pub fn tick(&mut self) {
		const NEIGHBOURS: [(isize, isize); 8] = [(-1, 1), (-1, 0), (-1, -1), (0, 1), (0, -1), (1, 1), (1, 0), (1, -1)];

		let rows = self.grid.len();
		let Some(cols) = self.grid.get(0).map(|items| items.len()) else {
			return;
		};

		for row in 0..rows {
			for col in 0..cols {
				let mut alive_count = 0;

				for (i, j) in &NEIGHBOURS {
					let (x, y) = (row as isize + *i, col as isize + *j);
					if x < 0 || y < 0 || x >= rows as isize || y >= cols as isize {
						continue;
					}

					let cell = &self.grid[x as usize][y as usize];
					if cell.alive {
						alive_count += 1;
					}
				}

				let cell = &mut self.grid[row][col];
				if cell.alive {
					if alive_count < 2 || alive_count > 3 {
						cell.alive = false;
					}
				} else {
					if alive_count == 3 {
						cell.alive = true;
					}
				}
			}
		}
	}
}
