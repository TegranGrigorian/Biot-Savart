use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Resource, Default)]
struct UiState {
    current: f32,
    wire_x: f32,
    wire_y: f32,
    wire_z: f32,
    probe_x: f32,
    probe_y: f32,
    probe_z: f32,
    add_wire_point_clicked: bool,
    set_probe_clicked: bool,
}

pub fn run_viewer() {
    App::new()
        .insert_resource(UiState {
            current: 1.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, ui_panel_system)
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
        ui.label("Current (A)");
        ui.add(egui::Slider::new(&mut ui_state.current, -20.0..=20.0));

        ui.separator();
        ui.label("Wire point");
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut ui_state.wire_x).speed(0.1).prefix("x: "));
            ui.add(egui::DragValue::new(&mut ui_state.wire_y).speed(0.1).prefix("y: "));
            ui.add(egui::DragValue::new(&mut ui_state.wire_z).speed(0.1).prefix("z: "));
        });
        if ui.button("Add wire point").clicked() {
            ui_state.add_wire_point_clicked = true;
        }

        ui.separator();
        ui.label("Probe point");
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut ui_state.probe_x).speed(0.1).prefix("x: "));
            ui.add(egui::DragValue::new(&mut ui_state.probe_y).speed(0.1).prefix("y: "));
            ui.add(egui::DragValue::new(&mut ui_state.probe_z).speed(0.1).prefix("z: "));
        });
        if ui.button("Set probe").clicked() {
            ui_state.set_probe_clicked = true;
        }
    });
}

fn apply_ui_actions_system(mut ui_state: ResMut<UiState>) {
    if ui_state.add_wire_point_clicked {
        let p = Vec3::new(ui_state.wire_x, ui_state.wire_y, ui_state.wire_z);
        info!("Add wire point: {:?}", p);
        // TODO: call your engine to append this point to the wire
        ui_state.add_wire_point_clicked = false;
    }

    if ui_state.set_probe_clicked {
        let p = Vec3::new(ui_state.probe_x, ui_state.probe_y, ui_state.probe_z);
        info!("Set probe point: {:?}", p);
        // TODO: call your engine to set probe point and compute B
        ui_state.set_probe_clicked = false;
    }
}