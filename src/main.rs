mod camera;
mod life;

use std::time::Duration;

use bevy::{color::palettes, core_pipeline::tonemapping::DebandDither, prelude::*, time::Stopwatch};
use camera::{orbit, zoom, zoom_interpolate, CameraSettings, OrbitCamera};
use life::{CellLocation, Conway, Destroy, LifeCell};
use rand::prelude::*;

const FIXED_TIMESTEP: f64 = 0.05;

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
	app.insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP));
	app.init_resource::<CameraSettings>();
	app.init_resource::<Handles>();
	app.init_resource::<Conway>();

	app.add_systems(Startup, setup);
	app.add_systems(FixedUpdate, tick_simulation);
	app.add_systems(
		Update,
		(
			orbit,
			(zoom, zoom_interpolate).chain(),
			(translate_cells, tick_destroy).chain(),
		),
	);

	app.run();
}

#[derive(Resource, Default)]
pub struct Handles {
	pub material: Handle<StandardMaterial>,
	pub mesh: Handle<Mesh>,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut handles: ResMut<Handles>,
	mut conway: ResMut<Conway>,
) {
	// Camera
	commands.spawn((
		Camera3d::default(),
		Camera { hdr: true, ..default() },
		DebandDither::Enabled,
		CameraSettings::init_transform(),
		OrbitCamera::default(),
	));

	// Lighting
	// commands.spawn(DirectionalLight {
	// 	shadows_enabled: false,
	// 	..default()
	// });
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

	let cube = Cuboid::from_length(conway.cube_size());
	let mesh_handle = meshes.add(cube);
	handles.mesh = mesh_handle.clone();

	// Conway board
	let mut rng = thread_rng();
	let mut new_cells = Vec::with_capacity(conway.grid_size().saturating_mul(conway.grid_size()));

	for row in 0..conway.grid_size() {
		for col in 0..conway.grid_size() {
			let alive = if rng.gen_range(1..=6) > 3 { true } else { false };
			if alive {
				new_cells.push((
					CellLocation::new(row, col, Duration::ZERO, &conway),
					Mesh3d(handles.mesh.clone()),
					MeshMaterial3d(handles.material.clone()),
				));
			}
			conway.current.push(LifeCell { row, col, alive });
		}
	}

	commands.spawn_batch(new_cells);
}

fn tick_simulation(mut commands: Commands, mut conway: ResMut<Conway>, handles: Res<Handles>) {
	conway.tick();

	let new_cells = conway
		.current
		.iter()
		.filter_map(|cell| {
			if cell.alive {
				Some((
					CellLocation::new(cell.row, cell.col, Duration::ZERO, &conway),
					Mesh3d(handles.mesh.clone()),
					MeshMaterial3d(handles.material.clone()),
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
	mut q_cells: Query<(&mut Transform, &mut CellLocation, Entity), Without<Destroy>>,
	time: Res<Time>,
) {
	const SPEED: f32 = 5.;
	const DESTROY_POSITION: f32 = 30.;

	for (mut transform, mut cell, entity) in &mut q_cells {
		cell.elapsed += time.delta();

		let timestep = Duration::from_secs_f64(FIXED_TIMESTEP + 0.01);
		if cell.elapsed < timestep {
			continue;
		}

		transform.translation.y += -SPEED * time.delta_secs();

		if transform.translation.y.abs() > DESTROY_POSITION {
			commands.entity(entity).insert(Destroy(Stopwatch::new()));
		}
	}
}

fn tick_destroy(mut commands: Commands, mut q_destroy: Query<(&mut Destroy, Entity)>, time: Res<Time>) {
	for (mut destroy, entity) in &mut q_destroy {
		if destroy.0.elapsed_secs() > 0.3 {
			commands.entity(entity).despawn_recursive();
		} else {
			destroy.0.tick(time.delta());
		}
	}
}
