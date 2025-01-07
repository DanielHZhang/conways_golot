use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Debug)]
pub struct Conway {
	pub current: Vec<LifeCell>,
	generation: usize,
	grid_size: usize,
	cube_size: f32,
}

#[derive(Debug, Clone)]
pub struct LifeCell {
	pub row: usize,
	pub col: usize,
	pub alive: bool,
}

impl Default for Conway {
	fn default() -> Self {
		let size: usize = 50;
		Self {
			current: Vec::with_capacity(size.saturating_mul(size)),
			grid_size: size,
			cube_size: 0.5,
			generation: 1,
		}
	}
}

impl Conway {
	pub fn tick(&mut self) {
		const NEIGHBOURS: [(isize, isize); 8] = [(-1, 1), (-1, 0), (-1, -1), (0, 1), (0, -1), (1, 1), (1, 0), (1, -1)];

		let mut new_cells = Vec::with_capacity(self.grid_size.saturating_mul(self.grid_size));

		for row in 0..self.grid_size {
			for col in 0..self.grid_size {
				let mut alive_count = 0;

				for (i, j) in &NEIGHBOURS {
					let x = row as isize + *i;
					let y = col as isize + *j;

					if x < 0 || y < 0 || x >= self.grid_size as isize || y >= self.grid_size as isize {
						continue;
					}

					let cell = &self.current[x as usize * self.grid_size + y as usize];
					if cell.alive {
						alive_count += 1;
					}
				}

				let mut cell = self.current[row * self.grid_size + col].clone();
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

	pub fn grid_size(&self) -> usize {
		self.grid_size
	}

	pub fn cube_size(&self) -> f32 {
		self.cube_size
	}
}

#[derive(Component)]
pub struct CellLocation {
	pub row: usize,
	pub col: usize,
	pub generation: usize,
	pub elapsed: Duration,
}

impl CellLocation {
	pub fn new(row: usize, col: usize, elapsed: Duration, conway: &Conway) -> (Self, Transform) {
		let &Conway {
			grid_size,
			cube_size,
			generation,
			..
		} = conway;
		let half_extent = -(grid_size as f32 * cube_size / 2.);
		let (x, z) = (row as f32 * cube_size, col as f32 * cube_size);
		(
			Self {
				row,
				col,
				generation,
				elapsed,
			},
			Transform::from_xyz(half_extent + x, 0., half_extent + z),
		)
	}
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Destroy(pub Stopwatch);
