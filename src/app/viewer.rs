use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

use crate::app::screen::UiState;
use crate::constants;

#[derive(Component)]
pub(crate) struct OrbitCamera {
	pub(crate) target: Vec3,
	pub(crate) radius: f32,
	pub(crate) yaw: f32,
	pub(crate) pitch: f32,
	pub(crate) rotate_sensitivity: f32,
	pub(crate) zoom_sensitivity: f32,
	pub(crate) pan_sensitivity: f32,
	pub(crate) last_cursor_pos: Option<Vec2>,
}

impl Default for OrbitCamera {
	fn default() -> Self {
		Self {
			target: Vec3::ZERO,
			radius: 12.0,
			yaw: -0.5,
			pitch: -0.4,
			rotate_sensitivity: 0.004,
			zoom_sensitivity: 0.12,
			pan_sensitivity: 0.0025,
			last_cursor_pos: None,
		}
	}
}

#[derive(Component)]
pub(crate) struct ProbeMarker;

#[derive(Component)]
pub(crate) struct SandboxGround;

fn current_arrow_length(current_mag: f32) -> f32 {
	(0.22 + current_mag.sqrt() * 0.28).clamp(0.22, 3.5)
}

fn b_arrow_length(b_mag: f32) -> f32 {
	let normalized = (b_mag / 1.0e-9).max(1.0e-6);
	(0.20 + normalized.powf(0.25) * 0.55).clamp(0.20, 4.5)
}

fn current_arrow(ui_state: &UiState) -> Option<(Vec3, Vec3, f32)> {
	if ui_state.wire_points.len() < 2 {
		return None;
	}

	let p0 = ui_state.wire_points[0];
	let p1 = ui_state.wire_points[1];
	let seg = Vec3::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
	if seg.length_squared() <= 1.0e-10 {
		return None;
	}

	let current_mag = ui_state.current.abs();
	if current_mag <= f32::EPSILON {
		return None;
	}

	let dir = seg.normalize() * ui_state.current.signum();
	let mid = Vec3::new(
		(p0.x + p1.x) * 0.5,
		(p0.y + p1.y) * 0.5,
		(p0.z + p1.z) * 0.5,
	);
	let arrow_len = current_arrow_length(current_mag);
	Some((
		mid - dir * (arrow_len * 0.5),
		mid + dir * (arrow_len * 0.5),
		current_mag,
	))
}

fn b_arrow(ui_state: &UiState) -> Option<(Vec3, Vec3, f32)> {
	let b_vec = ui_state.last_b_vec?;
	let b = Vec3::new(b_vec.x, b_vec.y, b_vec.z);
	let b_mag = b.length();
	if b_mag < constants::MIN_B_RENDER_MAG {
		return None;
	}

	let probe = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);
	let b_len = b_arrow_length(b_mag);
	Some((probe, probe + b.normalize() * b_len, b_mag))
}

fn sandbox_center_and_half_extent(ui_state: &UiState) -> (Vec3, f32) {
	let mut min = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);
	let mut max = min;

	for p in &ui_state.wire_points {
		let point = Vec3::new(p.x, p.y, p.z);
		min = min.min(point);
		max = max.max(point);
	}

	let center = (min + max) * 0.5;
	let raw_half = ((max - min) * 0.5).max_element();
	let padded_half = (raw_half * 1.35).max(6.0);
	(center, padded_half)
}

pub(crate) fn setup_viewer(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		Mesh3d(meshes.add(Sphere::new(0.06).mesh().uv(24, 16))),
		MeshMaterial3d(materials.add(Color::srgb(0.9, 0.25, 0.2))),
		Transform::from_xyz(0.0, 0.0, 0.0),
		ProbeMarker,
	));
}

pub(crate) fn draw_dynamic_viewer_system(ui_state: Res<UiState>, mut gizmos: Gizmos) {
	if ui_state.wire_points.len() >= 2 {
		for seg in ui_state.wire_points.windows(2) {
			let start = Vec3::new(seg[0].x, seg[0].y, seg[0].z);
			let end = Vec3::new(seg[1].x, seg[1].y, seg[1].z);
			gizmos.line(start, end, Color::srgb(1.0, 0.8, 0.2));
		}
	}

	if let Some((start, end, _)) = current_arrow(&ui_state) {
		gizmos.arrow(start, end, Color::srgb(0.2, 1.0, 0.2));
	}

	if let Some((start, end, _)) = b_arrow(&ui_state) {
		gizmos.arrow(start, end, Color::srgb(0.2, 0.8, 1.0));
	}
}

pub(crate) fn update_sandbox_ground_system(
	ui_state: Res<UiState>,
	mut ground_q: Query<&mut Transform, With<SandboxGround>>,
) {
	let (center, half_extent) = sandbox_center_and_half_extent(&ui_state);
	if let Ok(mut transform) = ground_q.single_mut() {
		let size = half_extent * 2.0;
		let plane_scale = size / constants::BASE_GROUND_SIZE;
		transform.translation = Vec3::new(center.x, 0.0, center.z);
		transform.scale = Vec3::new(plane_scale, 1.0, plane_scale);
	}
}

pub(crate) fn orbit_camera_system(
	mouse_buttons: Res<ButtonInput<MouseButton>>,
	keys: Res<ButtonInput<KeyCode>>,
	mut mouse_motion: MessageReader<MouseMotion>,
	mut mouse_wheel: MessageReader<MouseWheel>,
	primary_window_q: Query<&Window, With<PrimaryWindow>>,
	mut camera_q: Query<(&mut OrbitCamera, &mut Transform)>,
) {
	let mut motion_delta_messages = Vec2::ZERO;
	for ev in mouse_motion.read() {
		motion_delta_messages += ev.delta;
	}

	let mut scroll: f32 = 0.0;
	for ev in mouse_wheel.read() {
		scroll += ev.y;
	}

	let cursor_pos = primary_window_q
		.single()
		.ok()
		.and_then(|window| window.cursor_position());

	for (mut orbit, mut transform) in &mut camera_q {
		let cursor_delta = match (cursor_pos, orbit.last_cursor_pos) {
			(Some(current), Some(last)) => current - last,
			_ => Vec2::ZERO,
		};
		orbit.last_cursor_pos = cursor_pos;

		let motion_delta = if motion_delta_messages.length_squared() > 0.0 {
			motion_delta_messages
		} else {
			cursor_delta
		};

		if scroll.abs() > f32::EPSILON {
			let zoom_factor = 1.0 - scroll * orbit.zoom_sensitivity;
			orbit.radius = (orbit.radius * zoom_factor).clamp(0.5, 150.0);
		}

		let rotate_mode = mouse_buttons.pressed(MouseButton::Right)
			|| mouse_buttons.pressed(MouseButton::Left);

		if rotate_mode {
			orbit.yaw -= motion_delta.x * orbit.rotate_sensitivity;
			orbit.pitch = (orbit.pitch - motion_delta.y * orbit.rotate_sensitivity).clamp(-1.54, 1.54);
		}

		let pan_mode = mouse_buttons.pressed(MouseButton::Middle)
			|| ((mouse_buttons.pressed(MouseButton::Right) || mouse_buttons.pressed(MouseButton::Left))
				&& (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)));

		if pan_mode {
			let right = transform.rotation * Vec3::X;
			let up = transform.rotation * Vec3::Y;
			let pan_sensitivity = orbit.pan_sensitivity;
			let radius = orbit.radius;
			orbit.target += (-right * motion_delta.x + up * motion_delta.y) * pan_sensitivity * radius;
		}

		let rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
		transform.translation = orbit.target + rotation * Vec3::new(0.0, 0.0, orbit.radius);
		transform.look_at(orbit.target, Vec3::Y);
	}
}

pub(crate) fn update_viewer_entities_system(
	ui_state: Res<UiState>,
	mut probe_q: Query<&mut Transform, With<ProbeMarker>>,
) {
	let probe_pos = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);

	if let Ok(mut probe_transform) = probe_q.single_mut() {
		probe_transform.translation = probe_pos;
	}
}

pub(crate) fn draw_overlay_labels_system(
	mut contexts: EguiContexts,
	ui_state: Res<UiState>,
	camera_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) {
	let Ok((camera, camera_transform)) = camera_q.single() else {
		return;
	};

	let Ok(ctx) = contexts.ctx_mut() else {
		return;
	};

	let painter = ctx.layer_painter(egui::LayerId::new(
		egui::Order::Foreground,
		egui::Id::new("viewer_overlay_labels"),
	));
	let mut occupied: Vec<egui::Rect> = Vec::new();

	let mut draw_label = |world_pos: Vec3, text: String, color: egui::Color32| {
		if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
			let galley = painter.layout_no_wrap(text, egui::FontId::proportional(16.0), color);
			let mut pos = egui::pos2(screen_pos.x + 8.0, screen_pos.y - 8.0);
			let mut rect = egui::Rect::from_min_size(pos, galley.size());

			for _ in 0..8 {
				let overlaps = occupied
					.iter()
					.any(|used| used.expand(5.0).intersects(rect));
				if !overlaps {
					break;
				}
				pos.y += galley.size().y + 6.0;
				rect = egui::Rect::from_min_size(pos, galley.size());
			}

			painter.rect_filled(
				rect.expand2(egui::vec2(4.0, 2.0)),
				4.0,
				egui::Color32::from_black_alpha(110),
			);
			painter.galley(pos, galley, color);
			occupied.push(rect);
		}
	};

	if let Some(first) = ui_state.wire_points.first() {
		draw_label(
			Vec3::new(first.x, first.y, first.z) + Vec3::new(0.0, 0.22, 0.0),
			ui_state.wire_name.clone(),
			egui::Color32::from_rgb(255, 230, 170),
		);
	}

	let probe_pos = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);
	draw_label(
		probe_pos + Vec3::new(0.0, 0.18, 0.0),
		ui_state.probe_name.clone(),
		egui::Color32::from_rgb(175, 245, 255),
	);

	if let Some((start, end, current_mag)) = current_arrow(&ui_state) {
		draw_label(
			(start + end) * 0.5 + Vec3::new(0.0, 0.22, 0.0),
			format!("I = {:.3} A", if ui_state.current.is_sign_negative() { -current_mag } else { current_mag }),
			egui::Color32::from_rgb(160, 255, 160),
		);
	}

	if let Some((_, end, b_mag)) = b_arrow(&ui_state) {
		draw_label(
			end + Vec3::new(0.0, 0.20, 0.0),
			format!("|B| = {:.3e} T", b_mag),
			egui::Color32::from_rgb(130, 230, 255),
		);

		if let Some(b_vec) = ui_state.last_b_vec {
			draw_label(
				end + Vec3::new(0.0, 0.48, 0.0),
				format!(
					"B = ({:.2e}) i^ + ({:.2e}) j^ + ({:.2e}) k^ T",
					b_vec.x, b_vec.y, b_vec.z
				),
				egui::Color32::from_rgb(150, 235, 255),
			);
		}
	} else if let Some(b_vec) = ui_state.last_b_vec {
		draw_label(
			probe_pos + Vec3::new(0.0, 0.22, 0.0),
			format!(
				"B = ({:.2e}) i^ + ({:.2e}) j^ + ({:.2e}) k^ T",
				b_vec.x, b_vec.y, b_vec.z
			),
			egui::Color32::from_rgb(150, 235, 255),
		);
	}

	if let Some(err) = &ui_state.last_error {
		draw_label(
			probe_pos + Vec3::new(0.0, 0.38, 0.0),
			format!("B error: {}", err),
			egui::Color32::from_rgb(255, 130, 130),
		);
	}
}
