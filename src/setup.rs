use bevy::{
    prelude::*,
    audio::{PlaybackMode, PlaybackSettings, Volume}
};

use super::AppState;

#[derive(Asset, TypePath, Clone)]
pub struct Atlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct GameAssets {
    // ui
    pub font: Handle<Font>,
    pub background: Handle<Image>,
    pub menu_background: Handle<Image>,
    pub countdown: Atlas,
    pub red_alert: Atlas,
    pub life: Handle<Image>,
    pub game_over: Handle<Image>,
    pub you_won: Handle<Image>,

    // audio
    pub laser_sound: Handle<AudioSource>,

    // game
    pub player: Atlas,
    pub i_type: Handle<Image>,
    pub p_type: Handle<Image>,
    pub v_laser: Handle<Image>,
    pub h_laser: Handle<Image>,
    pub explosion: Atlas,
    pub big_explosion: Atlas,
    pub star_node_laser: Atlas,
    pub v_star: Handle<Image>,
    pub h_star: Handle<Image>,
    pub mine: Handle<Image>,
    pub asteroid: Handle<Image>,
    pub star_node_v1: Atlas,
    pub star_node_v2: Atlas,
    pub star_node_v3: Atlas,
    pub star_node_v4: Atlas,
    pub star_node_v5: Atlas,
    pub star_node_v6: Atlas,
    pub star_node_h1: Atlas,
    pub star_node_h2: Atlas,
    pub star_node_h3: Atlas,
    pub star_node_h4: Atlas,
    pub star_node_h5: Atlas,
    pub star_node_h6: Atlas,
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut app_state: ResMut<NextState<AppState>>
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/Test.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.0),
            ..default()
        },
        Name::from("Background Music"),
    ));

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
        player: Atlas {
            texture: asset_server.load("player.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        countdown: Atlas {
            texture: asset_server.load("countdown.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(256, 256),
                3,
                1,
                None,
                None,
            )),
        },
        red_alert: Atlas {
            texture: asset_server.load("red_alert.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(268, 32),
                2,
                1,
                None,
                None,
            )),
        },
        explosion: Atlas {
            texture: asset_server.load("explosion.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                3,
                1,
                None,
                None,
            )),
        },
        big_explosion: Atlas {
            texture: asset_server.load("big_explosion.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(256, 256),
                3,
                1,
                None,
                None,
            )),
        },
        star_node_laser: Atlas {
            texture: asset_server.load("star_node_laser.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(16, 16),
                4,
                1,
                None,
                None,
            )),
        },
        star_node_v1: Atlas {
            texture: asset_server.load("star_node_v1.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_v2: Atlas {
            texture: asset_server.load("star_node_v2.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_v3: Atlas {
            texture: asset_server.load("star_node_v3.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_v4: Atlas {
            texture: asset_server.load("star_node_v4.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_v5: Atlas {
            texture: asset_server.load("star_node_v5.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_v6: Atlas {
            texture: asset_server.load("star_node_v6.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_h1: Atlas {
            texture: asset_server.load("star_node_h1.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_h2: Atlas {
            texture: asset_server.load("star_node_h2.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_h3: Atlas {
            texture: asset_server.load("star_node_h3.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_h4: Atlas {
            texture: asset_server.load("star_node_h4.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_h5: Atlas {
            texture: asset_server.load("star_node_h5.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
        star_node_h6: Atlas {
            texture: asset_server.load("star_node_h6.png"),
            layout: layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(64, 64),
                2,
                1,
                None,
                None,
            )),
        },
    };

    commands.insert_resource(game_assets);

    app_state.set(AppState::Menu);
}
