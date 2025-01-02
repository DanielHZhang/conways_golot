use bevy::prelude::*;
use rand::prelude::*;

use crate::Handles;

#[derive(Resource, Debug)]
pub struct Conway {
	// pub entities: Vec<Entity>,
	pub current: Vec<LifeCell>,
	pub prev: Vec<LifeCell>,
	size: usize,
}

#[derive(Debug, Clone)]
pub struct LifeCell {
	pub row: usize,
	pub col: usize,
	pub alive: bool,
}

impl Conway {
	pub const CUBE_SIZE: f32 = 0.25;

	pub fn new(size: usize) -> Self {
		let capacity = size.saturating_mul(size);

		Self {
			// entities,
			prev: Vec::with_capacity(capacity),
			current: Vec::with_capacity(capacity),
			size,
		}
	}

	pub fn tick(&mut self) {
		const NEIGHBOURS: [(isize, isize); 8] = [(-1, 1), (-1, 0), (-1, -1), (0, 1), (0, -1), (1, 1), (1, 0), (1, -1)];

		std::mem::swap(&mut self.prev, &mut self.current); // Use prev to build new current

		for row in 0..self.size {
			for col in 0..self.size {
				let mut alive_count = 0;

				for (i, j) in &NEIGHBOURS {
					let x = row as isize + *i;
					let y = col as isize + *j;

					if x < 0 || y < 0 || x >= self.size as isize || y >= self.size as isize {
						continue;
					}

					let cell = &self.prev[x as usize * self.size + y as usize];
					if cell.alive {
						alive_count += 1;
					}
				}

				let cell = &mut self.current[row * self.size + col];
				cell.alive = match (cell.alive, alive_count) {
					(true, 2..=3) => true,
					(false, 3) => true,
					_ => false,
				};
			}
		}
	}
}

#[derive(Component, Clone, Hash)]
pub struct CellLocation {
	pub row: usize,
	pub col: usize,
	pub tick: usize,
}

#[derive(Component)]
pub struct TopLayer;
