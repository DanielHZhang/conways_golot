mod camera;
mod life;

use std::time::Duration;

use bevy::{color::palettes, core_pipeline::tonemapping::DebandDither, prelude::*, time::common_conditions::on_timer};
use camera::{orbit, CameraSettings};
use life::{Conway, LifeCell, LifeCellRef};
use rand::prelude::*;

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

	app.insert_resource(ClearColor(Color::BLACK));
	app.init_resource::<CameraSettings>();
	app.init_resource::<Handles>();

	app.add_systems(Startup, setup);
	app.add_systems(
		Update,
		(orbit, tick_simulation.run_if(on_timer(Duration::from_millis(200)))),
	);

	app.run();
}

#[derive(Resource, Default)]
struct Handles {
	material: Handle<StandardMaterial>,
	mesh: Handle<Mesh>,
}

const CUBE_SIZE: f32 = 1.;
const GRID_SIZE: usize = 20;

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
	let mesh_handle = meshes.add(Cuboid::from_length(CUBE_SIZE));
	handles.material = material_handle.clone();
	handles.mesh = mesh_handle.clone();

	// Conway board
	let mut conway = Conway::new(GRID_SIZE as u8);
	let mut rng = thread_rng();
	for row in 0..GRID_SIZE {
		for col in 0..GRID_SIZE {
			let alive = if rng.gen_range(1..=6) > 3 { true } else { false };
			let entity = commands
				.spawn((
					LifeCellRef { row, col, tick: 0 },
					Mesh3d(mesh_handle.clone()),
					MeshMaterial3d(material_handle.clone()),
					Transform::from_xyz(row as f32 * CUBE_SIZE, 0., col as f32 * CUBE_SIZE),
					if alive { Visibility::Visible } else { Visibility::Hidden },
				))
				.id();
			conway.grid[row][col] = LifeCell {
				entity: Some(entity),
				alive,
			};
		}
	}

	commands.insert_resource(conway);
}

fn tick_simulation(
	mut commands: Commands,
	mut q_cells: Query<(&mut LifeCellRef, &mut Transform, &mut Visibility)>,
	mut conway: ResMut<Conway>,
	handles: Res<Handles>,
) {
	// Before tick, clone the current state of the board
	for (cell, _, visibility) in q_cells.iter_many(
		conway
			.grid
			.iter()
			.flat_map(|cells| cells)
			.filter_map(|cell| cell.entity),
	) {
		if let Visibility::Hidden = visibility {
			continue; // Skip hidden cells
		}

		let LifeCellRef { row, col, .. } = cell.clone();
		commands.spawn((
			LifeCellRef { row, col, tick: 1 },
			Mesh3d(handles.mesh.clone()),
			MeshMaterial3d(handles.material.clone()),
			Transform::from_xyz(row as f32 * CUBE_SIZE, -1., col as f32 * CUBE_SIZE),
			Visibility::Visible,
		));
	}

	conway.tick();

	// After tick, update the state of the top slice of blocks
	let mut iter = q_cells.iter_many_mut(
		conway
			.grid
			.iter()
			.flat_map(|cells| cells)
			.filter_map(|cell| cell.entity),
	);
	while let Some((cell_ref, _, mut visibility)) = iter.fetch_next() {
		let cell = &conway.grid[cell_ref.row][cell_ref.col];

		if cell.alive && visibility.as_ref() == Visibility::Hidden {
			*visibility = Visibility::Visible;
		} else if !cell.alive && visibility.as_ref() == Visibility::Visible {
			*visibility = Visibility::Hidden;
		}
	}

	// Shift all states downwards
	for (mut cell, mut transform, _) in q_cells.iter_mut() {
		if cell.tick >= 1 {
			transform.translation.y -= 1.;
			cell.tick += 1;
		}
	}
}
