use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use nalgebra::Vector3;
use crate::app::{config, viewer};
use crate::engine::components::wire::Wire;
use crate::engine::components::point::Point;
use crate::engine::components::curve::ParametricCurve;
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
    pub(crate) debug_info: String,
    pub(crate) show_labels: bool,
    pub(crate) show_arrows: bool,
    pub(crate) dirty: bool,
    add_wire_point_clicked: bool,
    clear_wire_clicked: bool,
    set_probe_clicked: bool,
    pub(crate) param_x_expr: String,
    pub(crate) param_y_expr: String,
    pub(crate) param_z_expr: String,
    pub(crate) param_t_min: f32,
    pub(crate) param_t_max: f32,
    pub(crate) param_samples: u32,
    generate_parametric_clicked: bool,
}

// guess what, runs the viewer crazy ik
pub fn run_viewer() {
    App::new()
        .insert_resource(UiState {
            current: 1.0,
            wire_name: String::from("Wire 1"),
            probe_name: String::from("Probe"),
            show_labels: true,
            show_arrows: true,
            dirty: true,
            param_x_expr: String::from("cos(t)"),
            param_y_expr: String::from("sin(t)"),
            param_z_expr: String::from("0"),
            param_t_min: 0.0,
            param_t_max: 6.2832,
            param_samples: 200,
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
        .add_systems(EguiPrimaryContextPass, (ui_panel_system, viewer::draw_overlay_labels_system))
        .add_systems(Update, viewer::orbit_camera_system)
        .add_systems(Update, viewer::update_sandbox_ground_system)
        .add_systems(Update, viewer::draw_dynamic_viewer_system)
        .add_systems(Update, viewer::update_viewer_entities_system)
        .add_systems(Update, apply_ui_actions_system)
        .run();
}

// get default situated
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // default spawns, would put these in configs but you shouldnt ever change these they are just goofy values
    commands.spawn((
        Camera3d::default(),
        viewer::OrbitCamera::default(),
        Transform::from_xyz(6.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // let there be light!
    commands.spawn((
        PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(8.0, 12.0, 8.0),
    ));

    // if you dont want the floor you can comment the "commands.spawn" below, but tahts werid adn u should go see a therapist
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(config::PLANE_MESH_SIZE, config::PLANE_MESH_SIZE))),
        MeshMaterial3d(materials.add(config::ZERO_PLANE_COLOR)),
        viewer::SandboxGround,
    ));
}

// alot of this code kind of documents itself so im not going to add comments
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
            .add(egui::DragValue::new(&mut ui_state.current).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("A: "))
            .changed()
        {
            ui_state.dirty = true;
        }

        ui.separator();
        ui.label("Wire point");
        ui.horizontal(|ui| {
            if ui
                .add(egui::DragValue::new(&mut ui_state.wire_x).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("x: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.wire_y).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("y: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.wire_z).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("z: "))
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
        ui.collapsing("Parametric Wire", |ui| {
            ui.label("x(t) =");
            ui.text_edit_singleline(&mut ui_state.param_x_expr);
            ui.label("y(t) =");
            ui.text_edit_singleline(&mut ui_state.param_y_expr);
            ui.label("z(t) =");
            ui.text_edit_singleline(&mut ui_state.param_z_expr);
            ui.horizontal(|ui| {
                ui.label("t:");
                ui.add(egui::DragValue::new(&mut ui_state.param_t_min).speed(0.01).prefix("min: "));
                ui.add(egui::DragValue::new(&mut ui_state.param_t_max).speed(0.01).prefix("max: "));
            });
            ui.horizontal(|ui| {
                ui.label("Samples:");
                ui.add(egui::DragValue::new(&mut ui_state.param_samples).speed(1).range(2..=10000_u32));
            });
            if ui.button("Generate parametric wire").clicked() {
                ui_state.generate_parametric_clicked = true;
            }
            ui.label(egui::RichText::new("Functions: sin cos tan sqrt abs exp ln log floor ceil").small().weak());
            ui.label(egui::RichText::new("Constants: pi e    Ops: + - * / ^").small().weak());
        });

        ui.separator();
        ui.label("Probe name");
        if ui.text_edit_singleline(&mut ui_state.probe_name).changed() {
            ui_state.dirty = true;
        }

        ui.label("Probe point");
        ui.horizontal(|ui| {
            if ui
                .add(egui::DragValue::new(&mut ui_state.probe_x).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("x: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.probe_y).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("y: "))
                .changed()
            {
                ui_state.dirty = true;
            }
            if ui
                .add(egui::DragValue::new(&mut ui_state.probe_z).speed(config::DRAGABLE_TEXTBOX_SPEED).prefix("z: "))
                .changed()
            {
                ui_state.dirty = true;
            }
        });

        ui.separator();
        if let Some(b) = ui_state.last_b {
            ui.label(format!("|B| = {:.3e} T", b));
        }
        if let Some(b_vec) = ui_state.last_b_vec {
            ui.label(format!(
                "B = ({:.3e}) i^ + ({:.3e}) j^ + ({:.3e}) k^ T",
                b_vec.x, b_vec.y, b_vec.z
            ));
        }
        if let Some(err) = &ui_state.last_error {
            ui.colored_label(egui::Color32::RED, err);
        }

        ui.separator();
        if ui
            .button(if ui_state.show_labels {
                "Hide labels"
            } else {
                "Show labels"
            })
            .clicked()
        {
            ui_state.show_labels = !ui_state.show_labels;
        }
        if ui
            .button(if ui_state.show_arrows {
                "Hide arrows"
            } else {
                "Show arrows"
            })
            .clicked()
        {
            ui_state.show_arrows = !ui_state.show_arrows;
        }

        ui.separator();
        ui.collapsing("Debug", |ui| {
            if ui_state.debug_info.is_empty() {
                ui.label("No debug data yet.");
            } else {
                ui.monospace(ui_state.debug_info.clone());
            }
        });
    });
}

fn apply_ui_actions_system(mut ui_state: ResMut<UiState>) {
    if ui_state.clear_wire_clicked {
        ui_state.wire_points.clear();
        ui_state.last_b = None;
        ui_state.last_b_vec = None;
        ui_state.last_error = None;
        ui_state.debug_info.clear();
        ui_state.dirty = true;
        info!("Cleared wire points");
        ui_state.clear_wire_clicked = false;
    }

    if ui_state.generate_parametric_clicked {
        ui_state.generate_parametric_clicked = false;
        let result = ParametricCurve::new(
            ui_state.wire_name.clone(),
            &ui_state.param_x_expr,
            &ui_state.param_y_expr,
            &ui_state.param_z_expr,
            ui_state.param_t_min as f64,
            ui_state.param_t_max as f64,
        ).and_then(|curve| curve.sample(ui_state.param_samples as usize));
        match result {
            Ok(points) => {
                ui_state.wire_points = points;
                ui_state.last_error = None;
                ui_state.dirty = true;
                info!("Generated {} parametric wire points", ui_state.wire_points.len());
            }
            Err(err) => {
                ui_state.last_error = Some(format!("Parametric error: {err}"));
                info!("Parametric error: {err}");
            }
        }
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

        match Math::calculate_biot_savart_vector(wire, point, ui_state.current) { // error handling output, you shouldnt really want to read this but if you do, why? Are you doing ok?
            Ok(b_vec) => {
                let b_mag = b_vec.norm();
                let wire_length: f32 = if ui_state.wire_points.len() >= 2 {
                    ui_state
                        .wire_points
                        .windows(2)
                        .map(|w| (w[1] - w[0]).norm())
                        .sum()
                } else {
                    0.0
                };

                ui_state.last_b = Some(b_mag);
                ui_state.last_b_vec = Some(b_vec);
                ui_state.last_error = None;
                ui_state.debug_info = format!(
                    "points={} L={:.6} m I={:.3} A probe=({:.3},{:.3},{:.3}) B=({:.3e},{:.3e},{:.3e}) T |B|={:.3e} T",
                    ui_state.wire_points.len(),
                    wire_length,
                    ui_state.current,
                    ui_state.probe_x,
                    ui_state.probe_y,
                    ui_state.probe_z,
                    b_vec.x,
                    b_vec.y,
                    b_vec.z,
                    b_mag
                );
                info!("Compute debug: {}", ui_state.debug_info);
            }
            Err(err) => {
                ui_state.last_b = None;
                ui_state.last_b_vec = None;
                ui_state.last_error = Some(err.clone());
                ui_state.debug_info = format!(
                    "compute error with points={} I={:.3} A probe=({:.3},{:.3},{:.3}): {}",
                    ui_state.wire_points.len(),
                    ui_state.current,
                    ui_state.probe_x,
                    ui_state.probe_y,
                    ui_state.probe_z,
                    err
                );
                info!("Compute debug: {}", ui_state.debug_info);
            }
        }

        ui_state.dirty = false;
        ui_state.set_probe_clicked = false;
    }
}