use std::{
	f32::consts::{FRAC_PI_2, TAU},
	ops::Range,
};

use bevy::{
	input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
	prelude::*,
};

#[derive(Component)]
pub struct OrbitCamera {
	distance: f32,
	target_distance: f32,
}

impl Default for OrbitCamera {
	fn default() -> Self {
		Self {
			distance: Self::DEFAULT_DISTANCE,
			target_distance: Self::DEFAULT_DISTANCE,
		}
	}
}

impl OrbitCamera {
	const DEFAULT_DISTANCE: f32 = 50.;
}

#[derive(Debug, Resource)]
pub struct CameraSettings {
	pub orbit_range: Range<f32>,
	pub pitch_speed: f32,
	pub pitch_limit: f32,
	pub yaw_speed: f32,
}

impl Default for CameraSettings {
	fn default() -> Self {
		Self {
			orbit_range: 10_f32..70.,
			pitch_speed: 0.003,
			pitch_limit: FRAC_PI_2 - 0.01,
			yaw_speed: 0.004,
		}
	}
}

impl CameraSettings {
	const TARGET: Vec3 = Vec3::new(0., -15., 0.);

	pub fn init_transform() -> Transform {
		Transform::from_translation(Vec3::new(30., 13., 30.)).looking_at(Self::TARGET, Vec3::Y)
	}
}

pub fn orbit(
	mut q_camera: Query<(&mut Transform, &OrbitCamera), With<Camera>>,
	camera_settings: Res<CameraSettings>,
	mouse_buttons: Res<ButtonInput<MouseButton>>,
	mouse_motion: Res<AccumulatedMouseMotion>,
) {
	if mouse_motion.delta.abs_diff_eq(Vec2::ZERO, 0.01) {
		return;
	}
	if !mouse_buttons.pressed(MouseButton::Left) {
		return;
	}

	let delta_yaw = -mouse_motion.delta.x * camera_settings.yaw_speed;
	let delta_pitch = -mouse_motion.delta.y * camera_settings.pitch_speed;

	let (mut transform, orbit) = q_camera.single_mut();
	let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
	let pitch = (pitch + delta_pitch).clamp(-camera_settings.pitch_limit, camera_settings.pitch_limit);
	let mut yaw = yaw + delta_yaw;
	if yaw > TAU {
		yaw -= TAU;
	} else if yaw < 0. {
		yaw += TAU;
	}

	transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
	transform.translation = CameraSettings::TARGET - transform.forward() * orbit.distance;
}

pub fn zoom(
	mut q_camera: Query<&mut OrbitCamera, With<Camera>>,
	mouse_scroll: Res<AccumulatedMouseScroll>,
	settings: Res<CameraSettings>,
) {
	if mouse_scroll.delta.abs_diff_eq(Vec2::ZERO, 0.01) {
		return;
	}

	let mut orbit = q_camera.single_mut();
	let target_distance = orbit.target_distance - mouse_scroll.delta.y;
	if settings.orbit_range.contains(&target_distance) {
		orbit.target_distance = target_distance;
	}
}

pub fn zoom_interpolate(mut q_camera: Query<(&mut Transform, &mut OrbitCamera), With<Camera>>, time: Res<Time>) {
	let (mut transform, mut orbit) = q_camera.single_mut();
	if orbit.distance == orbit.target_distance {
		return;
	}

	const ZOOM_SPEED: f32 = 10.;
	let ease_out = |t: f32| 1. - 2_f32.powf(-ZOOM_SPEED * t);
	orbit.distance = orbit.distance.lerp(orbit.target_distance, ease_out(time.delta_secs()));
	transform.translation = CameraSettings::TARGET - transform.forward() * orbit.distance;
}
