mod camera;
mod life;

use bevy::{color::palettes, core_pipeline::tonemapping::DebandDither, prelude::*};
use camera::{orbit, CameraSettings};
use life::Conway;

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

	app.add_systems(Startup, setup);
	app.add_systems(Update, orbit);

	app.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
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

	// Conway board
	let conway = Conway::new(4);
	let alive_material = StandardMaterial {
		base_color: palettes::tailwind::ZINC_200.into(),
		..Default::default()
	};
	let material_handle = materials.add(alive_material);
	let mut grid_bundles = Vec::new();
	let cube_size = 1.;

	for (row, items) in conway.grid.iter().enumerate() {
		for (col, cell) in items.iter().enumerate() {
			if !cell.alive {
				continue;
			}
			grid_bundles.push((
				Mesh3d(meshes.add(Cuboid::from_length(cube_size))),
				MeshMaterial3d(material_handle.clone()),
				Transform::from_xyz(row as f32 * cube_size, 0., col as f32 * cube_size),
			))
		}
	}
	commands.spawn_batch(grid_bundles);
}
