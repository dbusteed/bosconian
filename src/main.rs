use bevy::{prelude::*, audio::{Volume, VolumeLevel}};
use bevy::window::{PresentMode, WindowResolution};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod classic;
mod endless;
mod levels;
mod menu;
mod game;

#[derive(Resource)]
pub struct GameAssets {
    // ui
    font: Handle<Font>,
    background: Handle<Image>,
    menu_background: Handle<Image>,
    countdown: Handle<TextureAtlas>,
    red_alert: Handle<TextureAtlas>,
    life: Handle<Image>,
    game_over: Handle<Image>,
    you_won: Handle<Image>,

    // audio
    laser_sound: Handle<AudioSource>,

    // game
    player: Handle<TextureAtlas>,
    i_type: Handle<Image>,
    p_type: Handle<Image>,
    v_laser: Handle<Image>,
    h_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
    big_explosion: Handle<TextureAtlas>,
    star_node_laser: Handle<TextureAtlas>,
    v_star: Handle<Image>,
    h_star: Handle<Image>,
    mine: Handle<Image>,
    asteroid: Handle<Image>,
    star_node_v1: Handle<TextureAtlas>,
    star_node_v2: Handle<TextureAtlas>,
    star_node_v3: Handle<TextureAtlas>,
    star_node_v4: Handle<TextureAtlas>,
    star_node_v5: Handle<TextureAtlas>,
    star_node_v6: Handle<TextureAtlas>,
    star_node_h1: Handle<TextureAtlas>,
    star_node_h2: Handle<TextureAtlas>,
    star_node_h3: Handle<TextureAtlas>,
    star_node_h4: Handle<TextureAtlas>,
    star_node_h5: Handle<TextureAtlas>,
    star_node_h6: Handle<TextureAtlas>,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Classic,
    Endless,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bosconian".into(),
                        resolution: WindowResolution::new(1000., 750.)
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
            // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            menu::MenuPlugin,
            classic::ClassicPlugin,
            endless::EndlessPlugin,
        ))
        .add_state::<AppState>()
        .add_systems(PreStartup, load_assets)
        .run();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Test.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::Relative(VolumeLevel::new(0.25)),
            ..default()
        },
        ..default()
    });

    let game_assets = GameAssets {
        life: asset_server.load("player_single.png"),
        font: asset_server.load("fonts/emulogic.ttf"),
        menu_background: asset_server.load("menu_background_2.png"),
        background: asset_server.load("background.png"),
        laser_sound: asset_server.load("sounds/laser5.ogg"),
        game_over: asset_server.load("game_over.png"),
        you_won: asset_server.load("you_won.png"),
        i_type: asset_server.load("i_type.png"),
        p_type: asset_server.load("p_type.png"),
        h_laser: asset_server.load("h_laser.png"),
        v_laser: asset_server.load("v_laser.png"),
        v_star: asset_server.load("v_star.png"),
        h_star: asset_server.load("h_star.png"),
        mine: asset_server.load("mine.png"),
        asteroid: asset_server.load("asteroid.png"),
        player: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("player.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        countdown: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("countdown.png"),
            Vec2::new(256.0, 256.0),
            3,
            1,
            None,
            None,
        )),
        red_alert: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("red_alert.png"),
            Vec2::new(268.0, 32.0),
            2,
            1,
            None,
            None,
        )),
        explosion: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("explosion.png"),
            Vec2::new(64.0, 64.0),
            3,
            1,
            None,
            None,
        )),
        big_explosion: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("big_explosion.png"),
            Vec2::new(256.0, 256.0),
            3,
            1,
            None,
            None,
        )),
        star_node_laser: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_laser.png"),
            Vec2::new(16.0, 16.0),
            4,
            1,
            None,
            None,
        )),
        star_node_v1: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_v1.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_v2: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_v2.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_v3: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_v3.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_v4: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_v4.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_v5: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_v5.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_v6: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_v6.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_h1: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_h1.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_h2: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_h2.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_h3: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_h3.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_h4: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_h4.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_h5: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_h5.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
        star_node_h6: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("star_node_h6.png"),
            Vec2::new(64.0, 64.0),
            2,
            1,
            None,
            None,
        )),
    };

    commands.insert_resource(game_assets);
}
