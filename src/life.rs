use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct Conway {
	pub grid: Vec<Vec<LifeCell>>,
}

#[derive(Component)]
pub struct LifeCell {
	pub generation: u32,
	pub alive: bool,
}

impl Conway {
	pub fn new(size: u8) -> Self {
		let mut grid = Vec::with_capacity(size as usize);
		let mut rng = thread_rng();

		for _ in 0..size {
			let mut col = Vec::with_capacity(size as usize);
			for _ in 0..size {
				col.push(LifeCell {
					generation: 0,
					alive: if rng.gen_range(1..=6) > 3 { true } else { false },
				})
			}
			grid.push(col);
		}

		Self { grid }
	}

	pub fn tick(&mut self) {
		//
	}
}
