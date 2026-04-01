use bevy::prelude::*;

use crate::app::screen::UiState;

#[derive(Component)]
struct ProbeMarker;

#[derive(Component)]
struct WireLabel;

#[derive(Component)]
struct ProbeLabel;

pub(crate) fn setup_viewer(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		Mesh3d(meshes.add(Cuboid::new(0.12, 0.12, 0.12))),
		MeshMaterial3d(materials.add(Color::srgb(0.9, 0.25, 0.2))),
		Transform::from_xyz(0.0, 0.0, 0.0),
		ProbeMarker,
	));

	commands.spawn((
		Text2d::new("Wire 1"),
		TextFont {
			font_size: 20.0,
			..Default::default()
		},
		TextColor(Color::srgb(1.0, 0.95, 0.7)),
		Transform::from_xyz(0.0, 0.35, 0.0),
		WireLabel,
	));

	commands.spawn((
		Text2d::new("Probe"),
		TextFont {
			font_size: 18.0,
			..Default::default()
		},
		TextColor(Color::srgb(0.8, 1.0, 1.0)),
		Transform::from_xyz(0.0, 0.25, 0.0),
		ProbeLabel,
	));
}

pub(crate) fn draw_dynamic_viewer_system(ui_state: Res<UiState>, mut gizmos: Gizmos) {
	if ui_state.wire_points.len() >= 2 {
		for seg in ui_state.wire_points.windows(2) {
			let start = Vec3::new(seg[0].x, seg[0].y, seg[0].z);
			let end = Vec3::new(seg[1].x, seg[1].y, seg[1].z);
			gizmos.line(start, end, Color::srgb(1.0, 0.8, 0.2));
		}

		let p0 = ui_state.wire_points[0];
		let p1 = ui_state.wire_points[1];
		let mut dir = Vec3::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
		if dir.length_squared() > 1.0e-8 {
			dir = dir.normalize() * ui_state.current.signum();
			let mid = Vec3::new(
				(p0.x + p1.x) * 0.5,
				(p0.y + p1.y) * 0.5,
				(p0.z + p1.z) * 0.5,
			);
			let arrow_len = (0.4 + ui_state.current.abs() * 0.1).clamp(0.25, 2.0);
			gizmos.arrow(mid - dir * (arrow_len * 0.5), mid + dir * (arrow_len * 0.5), Color::srgb(0.2, 1.0, 0.2));
		}
	}

	let probe = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);

	if let Some(b_vec) = ui_state.last_b_vec {
		let b = Vec3::new(b_vec.x, b_vec.y, b_vec.z);
		if b.length_squared() > 1.0e-16 {
			let b_dir = b.normalize();
			let b_len = (b.length() * 5.0e7).clamp(0.25, 2.5);
			gizmos.arrow(probe, probe + b_dir * b_len, Color::srgb(0.2, 0.8, 1.0));
		}
	}
}

pub(crate) fn update_viewer_entities_system(
	ui_state: Res<UiState>,
	mut probe_q: Query<&mut Transform, With<ProbeMarker>>,
	mut wire_label_q: Query<(&mut Transform, &mut Text2d), (With<WireLabel>, Without<ProbeLabel>)>,
	mut probe_label_q: Query<(&mut Transform, &mut Text2d), (With<ProbeLabel>, Without<WireLabel>)>,
) {
	let probe_pos = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);

	if let Ok(mut probe_transform) = probe_q.single_mut() {
		probe_transform.translation = probe_pos;
	}

	if let Ok((mut wire_label_transform, mut wire_text)) = wire_label_q.single_mut() {
		let wire_anchor = ui_state
			.wire_points
			.first()
			.map(|p| Vec3::new(p.x, p.y, p.z))
			.unwrap_or(Vec3::ZERO);
		wire_label_transform.translation = wire_anchor + Vec3::new(0.0, 0.35, 0.0);
		*wire_text = Text2d::new(ui_state.wire_name.clone());
	}

	if let Ok((mut probe_label_transform, mut probe_text)) = probe_label_q.single_mut() {
		probe_label_transform.translation = probe_pos + Vec3::new(0.0, 0.25, 0.0);
		*probe_text = Text2d::new(ui_state.probe_name.clone());
	}
}
