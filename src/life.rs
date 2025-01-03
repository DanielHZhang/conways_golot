use bevy::prelude::*;

use crate::CUBE_SIZE;

#[derive(Resource, Debug)]
pub struct Conway {
	pub current: Vec<LifeCell>,
	size: usize,
	generation: usize,
	pub allow_tick: bool,
}

#[derive(Debug, Clone)]
pub struct LifeCell {
	pub row: usize,
	pub col: usize,
	pub alive: bool,
}

impl Conway {
	pub fn new(size: usize) -> Self {
		Self {
			current: Vec::with_capacity(size.saturating_mul(size)),
			allow_tick: false,
			size,
			generation: 1,
		}
	}

	pub fn tick(&mut self) {
		const NEIGHBOURS: [(isize, isize); 8] = [(-1, 1), (-1, 0), (-1, -1), (0, 1), (0, -1), (1, 1), (1, 0), (1, -1)];

		let mut new_cells = Vec::with_capacity(self.size.saturating_mul(self.size));

		for row in 0..self.size {
			for col in 0..self.size {
				let mut alive_count = 0;

				for (i, j) in &NEIGHBOURS {
					let x = row as isize + *i;
					let y = col as isize + *j;

					if x < 0 || y < 0 || x >= self.size as isize || y >= self.size as isize {
						continue;
					}

					let cell = &self.current[x as usize * self.size + y as usize];
					if cell.alive {
						alive_count += 1;
					}
				}

				let mut cell = self.current[row * self.size + col].clone();
				cell.alive = match (cell.alive, alive_count) {
					(true, 2..=3) => true,
					(false, 3) => true,
					_ => false,
				};
				new_cells.push(cell);
			}
		}

		self.current = new_cells;
		self.generation += 1;
	}

	pub fn generation(&self) -> usize {
		self.generation
	}
}

#[derive(Component, Clone)]
pub struct CellLocation {
	pub row: usize,
	pub col: usize,
	pub gen: usize,
	pub elapsed: f32,
}

impl CellLocation {
	pub fn new_transform(&self, grid_size: usize) -> Transform {
		let half_extent = -(grid_size as f32 * CUBE_SIZE / 2.);
		let (row, col) = (self.row as f32 * CUBE_SIZE, self.col as f32 * CUBE_SIZE);
		Transform::from_xyz(half_extent + row, 0., half_extent + col)
	}
}
