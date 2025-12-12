use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution}
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod classic;
mod endless;
mod game;
mod levels;
mod menu;
mod setup;

pub use setup::{GameAssets, Atlas};


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    Menu,
    Classic,
    Endless,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bosconian".into(),
                        resolution: WindowResolution::new(1000, 750)
                            .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoVsync,
                        prevent_default_event_handling: false,
                        // fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            ShapePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            // bevy::diagnostic::LogDiagnosticsPlugin::default(),
            // bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            // EguiPlugin::default(),
            // WorldInspectorPlugin::new(),
            setup::SetupPlugin,
            menu::MenuPlugin,
            classic::ClassicPlugin,
            endless::EndlessPlugin,
        ))
        .init_state::<AppState>()
        .run();
}
