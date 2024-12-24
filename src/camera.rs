use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Debug, Resource)]
pub struct CameraSettings {
	pub orbit_distance: f32,
	pub pitch_speed: f32,
	pub pitch_limit: f32,
	pub yaw_speed: f32,
}

impl Default for CameraSettings {
	fn default() -> Self {
		Self {
			orbit_distance: Self::ORBIT_DIST,
			pitch_speed: 0.003,
			pitch_limit: FRAC_PI_2 - 0.01,
			yaw_speed: 0.004,
		}
	}
}

impl CameraSettings {
	const ORBIT_DIST: f32 = 20.;

	pub fn init_transform() -> Transform {
		let position = Vec3::new(1., 1., 1.).normalize() * 20.;
		Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y)
	}
}

pub fn orbit(
	mut camera: Single<&mut Transform, With<Camera>>,
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

	let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
	let pitch = (pitch + delta_pitch).clamp(-camera_settings.pitch_limit, camera_settings.pitch_limit);
	let yaw = yaw + delta_yaw;
	let target = Vec3::ZERO;

	camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
	camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}
