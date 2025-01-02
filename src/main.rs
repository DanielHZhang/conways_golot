mod camera;
mod life;

use std::time::Duration;

use bevy::{color::palettes, core_pipeline::tonemapping::DebandDither, prelude::*, time::common_conditions::on_timer};
use camera::{orbit, CameraSettings};
use life::{CellLocation, Conway, LifeCell};
use rand::prelude::*;

#[derive(Resource, Default)]
struct Handles {
	material: Handle<StandardMaterial>,
	mesh: Handle<Mesh>,
}

pub const CUBE_SIZE: f32 = 0.25;

fn main() {
	let mut app = App::new();

	app.add_plugins(DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: "Conway's Game of Life".to_string(),
			resizable: false,
			resolution: (1280., 720.).into(),
			..default()
		}),
		..default()
	}));

	app.insert_resource(ClearColor(Color::srgb_u8(21, 20, 28)));
	app.init_resource::<CameraSettings>();
	app.init_resource::<Handles>();

	app.add_systems(Startup, setup);
	app.add_systems(
		Update,
		(
			tick_simulation.run_if(on_timer(Duration::from_millis(75))),
			translate_cells,
		)
			.chain(),
	);
	app.add_systems(Update, orbit);

	app.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut handles: ResMut<Handles>,
) {
	// Camera
	commands.spawn((
		Camera3d::default(),
		Camera { hdr: true, ..default() },
		DebandDither::Enabled,
		CameraSettings::init_transform(),
	));

	// Lighting
	commands.spawn(DirectionalLight {
		shadows_enabled: false,
		..default()
	});
	commands.spawn((
		PointLight {
			shadows_enabled: true,
			intensity: 10_000_000.,
			range: 100.0,
			shadow_depth_bias: 0.2,
			..default()
		},
		Transform::from_xyz(8.0, 16.0, 8.0),
	));

	let alive_material = StandardMaterial {
		base_color: palettes::tailwind::ZINC_200.into(),
		..Default::default()
	};
	let material_handle = materials.add(alive_material);
	handles.material = material_handle.clone();

	let cube = Cuboid::from_length(CUBE_SIZE);
	let mesh_handle = meshes.add(cube);
	handles.mesh = mesh_handle.clone();

	// Conway board
	let size = 50;
	let mut conway = Conway::new(size);
	let mut rng = thread_rng();
	let mut new_cells = Vec::with_capacity(size.saturating_mul(size));

	for row in 0..size {
		for col in 0..size {
			let alive = if rng.gen_range(1..=6) > 3 { true } else { false };
			if alive {
				new_cells.push((
					CellLocation {
						row,
						col,
						gen: conway.generation(),
					},
					Transform::from_xyz(row as f32 * CUBE_SIZE, 0., col as f32 * CUBE_SIZE),
					Mesh3d(handles.mesh.clone()),
					MeshMaterial3d(handles.material.clone()),
				));
			}
			conway.current.push(LifeCell { row, col, alive });
		}
	}

	commands.spawn_batch(new_cells);
	commands.insert_resource(conway);
}

fn tick_simulation(mut commands: Commands, mut conway: ResMut<Conway>, handles: Res<Handles>) {
	if !conway.allow_tick {
		return;
	}
	conway.tick();
	conway.allow_tick = false;

	let new_cells = conway
		.current
		.iter()
		.filter_map(|cell| {
			if cell.alive {
				Some((
					CellLocation {
						row: cell.row,
						col: cell.col,
						gen: conway.generation(),
					},
					Mesh3d(handles.mesh.clone()),
					MeshMaterial3d(handles.material.clone()),
					Transform::from_xyz(cell.row as f32 * CUBE_SIZE, 0., cell.col as f32 * CUBE_SIZE),
				))
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	commands.spawn_batch(new_cells);
}

fn translate_cells(
	mut commands: Commands,
	mut q_cells: Query<(&mut Transform, &CellLocation, Entity)>,
	mut conway: ResMut<Conway>,
	time: Res<Time>,
) {
	const SPEED: f32 = 5.;
	const DESTROY_POS: f32 = 20.;

	for (mut transform, cell, entity) in &mut q_cells {
		transform.translation.y += -SPEED * time.delta_secs();
		if !conway.allow_tick && cell.gen == conway.generation() && transform.translation.y <= -(CUBE_SIZE + 0.05) {
			conway.allow_tick = true;
		}

		if transform.translation.y.abs() > DESTROY_POS {
			commands.entity(entity).despawn_recursive();
		}
	}
}
