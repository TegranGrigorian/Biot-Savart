use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use nalgebra::Vector3;
use crate::app::viewer;
use crate::engine::components::wire::Wire;
use crate::engine::components::point::Point;
use crate::engine::math::Math;

#[derive(Resource, Default)]
pub(crate) struct UiState {
    pub(crate) current: f32,
    pub(crate) wire_name: String,
    pub(crate) wire_x: f32,
    pub(crate) wire_y: f32,
    pub(crate) wire_z: f32,
    pub(crate) wire_points: Vec<Vector3<f32>>,
    pub(crate) probe_name: String,
    pub(crate) probe_x: f32,
    pub(crate) probe_y: f32,
    pub(crate) probe_z: f32,
    pub(crate) last_b: Option<f32>,
    pub(crate) last_b_vec: Option<Vector3<f32>>,
    pub(crate) last_error: Option<String>,
    pub(crate) dirty: bool,
    add_wire_point_clicked: bool,
    clear_wire_clicked: bool,
    set_probe_clicked: bool,
}

pub fn run_viewer() {
    App::new()
        .insert_resource(UiState {
            current: 1.0,
            wire_name: String::from("Wire 1"),
            probe_name: String::from("Probe"),
            dirty: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Biot-Savart".to_string(),
                resolution: (1280, 720).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_scene, viewer::setup_viewer))
        .add_systems(EguiPrimaryContextPass, ui_panel_system)
        .add_systems(Update, viewer::orbit_camera_system)
        .add_systems(Update, viewer::draw_dynamic_viewer_system)
        .add_systems(Update, viewer::update_viewer_entities_system)
        .add_systems(Update, apply_ui_actions_system)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        viewer::OrbitCamera::default(),
        Transform::from_xyz(6.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(8.0, 12.0, 8.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.15, 0.15, 0.18))),
    ));
}

fn ui_panel_system(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
    egui::SidePanel::left("controls").show(contexts.ctx_mut().unwrap(), |ui| {
        ui.heading("Biot-Savart Controls");

        ui.separator();
        ui.label("Wire name");
        if ui.text_edit_singleline(&mut ui_state.wire_name).changed() {
            ui_state.dirty = true;
        }

        ui.label("Current (A)");
        if ui
            .add(egui::DragValue::new(&mut ui_state.current).speed(0.1).prefix("A: "))
            .changed()
        {
            ui_state.dirty = true;
        }

        ui.label("(Current arrow updates live)");

        ui.separator();
        ui.label("Wire point");
        ui.horizontal(|ui| {
            if ui
                .add(egui::DragValue::new(&mut ui_state.wire_x).speed(0.1).prefix("x: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.wire_y).speed(0.1).prefix("y: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.wire_z).speed(0.1).prefix("z: "))
                .changed()
            {
                ui_state.dirty = true;
            }
        });
        if ui.button("Add wire point").clicked() {
            ui_state.add_wire_point_clicked = true;
        }
        if ui.button("Clear wire").clicked() {
            ui_state.clear_wire_clicked = true;
        }
        ui.label(format!("Wire points: {}", ui_state.wire_points.len()));

        ui.separator();
        ui.label("Probe name");
        if ui.text_edit_singleline(&mut ui_state.probe_name).changed() {
            ui_state.dirty = true;
        }

        ui.label("Probe point");
        ui.horizontal(|ui| {
            if ui
                .add(egui::DragValue::new(&mut ui_state.probe_x).speed(0.1).prefix("x: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.probe_y).speed(0.1).prefix("y: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.probe_z).speed(0.1).prefix("z: "))
                .changed()
            {
                ui_state.dirty = true;
            }
        });
        if ui.button("Recompute now").clicked() {
            ui_state.set_probe_clicked = true;
        }

        ui.separator();
        if let Some(b) = ui_state.last_b {
            ui.label(format!("|B| = {:.3e} T", b));
        }
        if let Some(err) = &ui_state.last_error {
            ui.colored_label(egui::Color32::RED, err);
        }
    });
}

fn apply_ui_actions_system(mut ui_state: ResMut<UiState>) {
    if ui_state.clear_wire_clicked {
        ui_state.wire_points.clear();
        ui_state.last_b = None;
        ui_state.last_b_vec = None;
        ui_state.last_error = None;
        ui_state.dirty = true;
        info!("Cleared wire points");
        ui_state.clear_wire_clicked = false;
    }

    if ui_state.add_wire_point_clicked {
        let p = Vector3::new(ui_state.wire_x, ui_state.wire_y, ui_state.wire_z);
        ui_state.wire_points.push(p);
        ui_state.dirty = true;
        info!("Added wire point: [{:.3}, {:.3}, {:.3}]", p.x, p.y, p.z);
        ui_state.add_wire_point_clicked = false;
    }

    if ui_state.set_probe_clicked || ui_state.dirty {
        let wire = Wire::new(ui_state.wire_name.clone(), ui_state.wire_points.clone());
        let point = Point::new(
            ui_state.probe_name.clone(),
            Vector3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z),
        );

        match Math::calculate_biot_savart_vector(wire, point, ui_state.current) {
            Ok(b_vec) => {
                ui_state.last_b = Some(b_vec.norm());
                ui_state.last_b_vec = Some(b_vec);
                ui_state.last_error = None;
            }
            Err(err) => {
                ui_state.last_b = None;
                ui_state.last_b_vec = None;
                ui_state.last_error = Some(err.clone());
            }
        }

        ui_state.dirty = false;
        ui_state.set_probe_clicked = false;
    }
}