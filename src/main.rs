mod camera;
mod life;

use std::time::Duration;

use bevy::{color::palettes, core_pipeline::tonemapping::DebandDither, prelude::*, time::common_conditions::on_timer};
use camera::{orbit, CameraSettings};
use life::{CellLocation, Conway, LifeCell, TopLayer};
use rand::prelude::*;

#[derive(Resource, Default)]
struct Handles {
	material: Handle<StandardMaterial>,
	mesh: Handle<Mesh>,
}

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
	// app.add_systems(FixedUpdate, tick_simulation);
	app.add_systems(PreUpdate, tick_simulation.run_if(on_timer(Duration::from_millis(1000))));
	app.add_systems(Update, (orbit, translate_cells));

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
	let cube = Cuboid::from_length(Conway::CUBE_SIZE);
	let mesh_handle = meshes.add(cube);
	handles.material = material_handle.clone();
	handles.mesh = mesh_handle.clone();

	// Conway board
	let size = 50;
	let mut conway = Conway::new(size);
	let mut rng = thread_rng();

	for row in 0..size {
		for col in 0..size {
			let alive = if rng.gen_range(1..=6) > 3 { true } else { false };
			if alive {
				let entity = commands
					.spawn((
						// TopLayer,
						CellLocation { row, col, tick: 0 },
						Transform::from_xyz(row as f32 * Conway::CUBE_SIZE, 0., col as f32 * Conway::CUBE_SIZE),
						Mesh3d(handles.mesh.clone()),
						MeshMaterial3d(handles.material.clone()),
					))
					.id();
				// conway.entities.push(entity);
			}
			conway.current.push(LifeCell { row, col, alive });
		}
	}

	conway.prev = conway.current.clone();
	commands.insert_resource(conway);
}

fn tick_simulation(
	mut commands: Commands,
	// mut q_cells: Query<(&mut CellLocation), With<TopLayer>>,
	mut conway: ResMut<Conway>,
	handles: Res<Handles>,
) {
	// for entity in conway.entities.iter() {
	// 	commands.entity(*entity).remove::<TopLayer>();
	// }

	conway.tick();

	for (index, cell) in conway.current.iter().enumerate() {
		if !cell.alive {
			continue;
		}

		// println!("{index} {} {}", cell.row, cell.col);

		let entity = commands
			.spawn((
				// TopLayer,
				CellLocation {
					row: cell.row,
					col: cell.col,
					tick: 1,
				},
				Mesh3d(handles.mesh.clone()),
				MeshMaterial3d(handles.material.clone()),
				Transform::from_xyz(
					cell.row as f32 * Conway::CUBE_SIZE,
					0.,
					cell.col as f32 * Conway::CUBE_SIZE,
				),
			))
			.id();
	}
}

fn translate_cells(
	mut commands: Commands,
	mut q_cells: Query<(&mut Transform, Entity), (With<CellLocation>, Without<TopLayer>)>,
	time: Res<Time>,
) {
	const SPEED: f32 = 5.;
	const DESTROY_POS: f32 = 20.;

	for (mut transform, entity) in q_cells.iter_mut() {
		transform.translation.y += -SPEED * time.delta_secs();

		if transform.translation.y.abs() > DESTROY_POS {
			commands.entity(entity).despawn_recursive();
		}
	}
}
