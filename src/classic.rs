use bevy::{
    // core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use std::{f32::consts::PI, time::Duration};

use super::{
    game::{
        animation, bullet_timer, button_system, despawn_finished_sound_effects, destroy_game,
        follow_camera, listen_explosion, move_enemy_ships, player_input, star_node_shoot,
        update_minimap, world_to_minimap, Animation, CameraOffset, Collidable, Countdown,
        CountdownText, EnemyShip, EnemyType, Explodable, ExplodableType, ExplosionEvent,
        ExplosionSize, GameButton, GameButtonAction, GameCamera, GameNode, IType, LevelNode,
        MinimapCamera, MinimapPlayer, MinimapStar, PType, Player, PlayerDeathEvent, SetupLevel,
        StarCore, StarNode,
    },
    levels, AppState, Atlas, GameAssets,
};

struct Level {
    stars: Vec<levels::Star>,
    rocks: Vec<levels::Rock>,
    start_i: usize,
    max_i: usize,
    start_p: usize,
    max_p: usize,
    time_limit: usize,
}

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct Lives;

#[derive(Component)]
struct RedAlert;

#[derive(Component)]
struct SetupTimer(Timer);

#[derive(Component)]
struct ITypeTimer(Timer);

#[derive(Event)]
struct UpdateLivesEvent;

#[derive(Resource)]
struct Levels(Vec<Level>);

#[derive(Resource)]
struct Game {
    level: usize,
    lives: usize,
    countdown: usize,
    setup: bool,
    level_start_seconds: f32,
    red_alert: bool,
    itype_timer: Timer,
    ptype_timer: Timer,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum ClassicGameState {
    #[default]
    None,
    Setup,
    Countdown,
    Play,
    GameOver,
}

pub struct ClassicPlugin;
impl Plugin for ClassicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ClassicGameState::None)
            .add_event::<ExplosionEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<UpdateLivesEvent>()
            .add_event::<SetupLevel>()
            .add_systems(OnEnter(AppState::Classic), setup_game)
            .add_systems(OnExit(AppState::Classic), destroy_game)
            .add_systems(OnEnter(ClassicGameState::GameOver), setup_gameover)
            .add_systems(
                Update,
                button_system.run_if(in_state(ClassicGameState::GameOver)),
            )
            .add_systems(
                Update,
                countdown.run_if(in_state(ClassicGameState::Countdown)),
            )
            .add_systems(
                Update,
                (
                    player_input,
                    follow_camera,
                    check_collisions,
                    bullet_timer,
                    spawn_enemy_ships,
                    move_enemy_ships,
                    star_node_shoot,
                    star_update,
                    update_minimap,
                    despawn_finished_sound_effects,
                )
                    .run_if(in_state(ClassicGameState::Play)),
            )
            .add_systems(
                Update,
                (
                    animation,
                    listen_update_lives,
                    listen_player_death_classic,
                    listen_explosion,
                )
                    .run_if(not(in_state(ClassicGameState::None))),
            )
            .add_systems(
                Update,
                setup_level.run_if(in_state(ClassicGameState::Setup)),
            );
    }
}

fn setup_game(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<NextState<ClassicGameState>>,
    mut level_events: EventWriter<SetupLevel>,
) {
    rapier_config.gravity = Vec2::ZERO;

    // game camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            camera: Camera {
                order: 0,
                ..default()
            },
            // camera_2d: Camera2d {
            //     clear_color: ClearColorConfig::None,
            // },
            projection: OrthographicProjection {
                scale: 1.25,
                ..default()
            },
            ..default()
        },
        // UiCameraConfig { show_ui: true },
        GameCamera,
        RenderLayers::from_layers(&[0]),
        GameNode,
    ));

    // minimap / ui camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(1000 - 250, 750 - 250),
                    physical_size: UVec2::new(250, 250),
                    ..default()
                }),
                ..default()
            },
            // camera_2d: Camera2d {
            //     clear_color: ClearColorConfig::None,
            // },
            ..default()
        },
        // UiCameraConfig { show_ui: false },
        MinimapCamera,
        RenderLayers::from_layers(&[1]),
        GameNode,
    ));

    // lives camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 2,
                // viewport: Some(Viewport {
                //     physical_position: UVec2::new(1000 - (46 * 4) - 5, 5),
                //     physical_size: UVec2::new(46 * 4, 46),
                //     ..default()
                // }),
                ..default()
            },
            // camera_2d: Camera2d {
            //     clear_color: ClearColorConfig::None,
            // },
            ..default()
        },
        // UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[2]),
        GameNode,
    ));

    // background tiles
    for x in (-3000..=3000).step_by(1000) {
        for y in (-3000..=3000).step_by(1000) {
            commands.spawn((
                SpriteBundle {
                    texture: game_assets.background.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    ..default()
                },
                RenderLayers::layer(0),
                GameNode,
            ));
        }
    }

    // game boundary
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(5000.0, 5000.0),
                origin: RectangleOrigin::Center,
            }),
            ..default()
        },
        Fill::color(Color::srgba(0f32, 0f32, 0f32, 0f32)),
        Stroke::new(Color::srgb(1f32, 0f32, 0f32), 10.0),
        Collider::compound(vec![
            (Vec2::new(0., 2500.), 0f32, Collider::cuboid(2500., 2.5)),
            (Vec2::new(0., -2500.), 0f32, Collider::cuboid(2500., 2.5)),
            (Vec2::new(2500., 0.), 0f32, Collider::cuboid(2.5, 2500.)),
            (Vec2::new(-2500., 0.), 0f32, Collider::cuboid(2.5, 2500.)),
        ]),
        Collidable,
        RenderLayers::layer(0),
        GameNode,
    ));

    // level text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Level ",
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "1",
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        LevelText,
        GameNode,
    ));

    // minimap player
    commands.spawn((
        ShapeBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            },
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 5f32,
                center: Vec2::ZERO,
            }),
            ..default()
        },
        Fill::color(Color::WHITE),
        MinimapPlayer,
        RenderLayers::layer(1),
        GameNode,
    ));

    // Game resource
    commands.insert_resource(Game {
        level: 0,
        lives: 2, // DEBUG
        countdown: 3,
        setup: false,
        level_start_seconds: 0.0,
        red_alert: false,
        itype_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        ptype_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });

    commands.insert_resource(Levels(vec![
        Level {
            stars: levels::LEVEL_1.stars.to_vec(),
            rocks: levels::LEVEL_1.rocks.to_vec(),
            start_i: levels::LEVEL_1.start_i,
            max_i: levels::LEVEL_1.max_i,
            start_p: levels::LEVEL_1.start_p,
            max_p: levels::LEVEL_1.max_p,
            time_limit: levels::LEVEL_1.time_limit,
        },
        Level {
            stars: levels::LEVEL_2.stars.to_vec(),
            rocks: levels::LEVEL_2.rocks.to_vec(),
            start_i: levels::LEVEL_2.start_i,
            max_i: levels::LEVEL_2.max_i,
            start_p: levels::LEVEL_2.start_p,
            max_p: levels::LEVEL_2.max_p,
            time_limit: levels::LEVEL_2.time_limit,
        },
        Level {
            stars: levels::LEVEL_3.stars.to_vec(),
            rocks: levels::LEVEL_3.rocks.to_vec(),
            start_i: levels::LEVEL_3.start_i,
            max_i: levels::LEVEL_3.max_i,
            start_p: levels::LEVEL_3.start_p,
            max_p: levels::LEVEL_3.max_p,
            time_limit: levels::LEVEL_3.time_limit,
        },
    ]));

    commands.spawn((
        Countdown(Timer::from_seconds(0.01, TimerMode::Repeating)),
        GameNode,
    ));
    commands.spawn((
        SetupTimer(Timer::from_seconds(0.01, TimerMode::Repeating)),
        GameNode,
    ));

    game_state.set(ClassicGameState::Setup);
    level_events.send(SetupLevel);
}

fn setup_gameover(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game: Res<Game>,
    q_camera: Query<&mut Transform, With<GameCamera>>,
) {
    let camera = q_camera.get_single().unwrap();
    let trans = camera.translation + Vec3::new(0.0, 100.0, -50.0);
    let texture = if game.level > levels::MAX_LEVEL {
        game_assets.you_won.clone()
    } else {
        game_assets.game_over.clone()
    };

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: trans,
                ..default()
            },
            ..default()
        },
        GameNode,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(400.0),
                    width: Val::Px(1000.0),
                    height: Val::Px(350.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            GameNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(225.0),
                            height: Val::Px(70.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    },
                    GameButton {
                        action: GameButtonAction::ReturnToMenu,
                        idle_color: Color::srgb(0.15, 0.15, 0.15),
                        hover_color: Color::srgb(0.25, 0.25, 0.25),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Menu",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 30.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn setup_level(
    mut level_event: EventReader<SetupLevel>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<ClassicGameState>>,
    mut minimap: Query<&mut Camera, With<MinimapCamera>>,
    mut q_mm_player: Query<&mut Transform, With<MinimapPlayer>>,
) {
    for _ in level_event.read() {
        game.level += 1;
        game.setup = true;

        let mut cam = minimap.get_single_mut().unwrap();
        cam.is_active = false;

        for mut mm_trans in q_mm_player.iter_mut() {
            mm_trans.translation = world_to_minimap(Vec3::ZERO);
        }

        if game.level > levels::MAX_LEVEL {
            game_state.set(ClassicGameState::GameOver);
        } else {
            game_state.set(ClassicGameState::Countdown);
        }
    }
}

fn countdown(
    mut commands: Commands,
    time: Res<Time>,
    levels: Res<Levels>,
    game_assets: Res<GameAssets>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<ClassicGameState>>,
    mut life_events: EventWriter<UpdateLivesEvent>,
    q_countdown_text: Query<Entity, With<CountdownText>>,
    q_red_alert: Query<Entity, With<RedAlert>>,
    q_i_type: Query<Entity, (With<IType>, Without<PType>)>,
    q_p_type: Query<Entity, (With<PType>, Without<IType>)>,
    mut q_countdown: Query<&mut Countdown>,
    mut q_player: Query<&mut Velocity, With<Player>>,
    mut q_camera: Query<&mut Transform, With<GameCamera>>,
    mut q_minimap: Query<&mut Camera, With<MinimapCamera>>,
    mut q_level_text: Query<&mut Text, With<LevelText>>,
) {
    if let Ok(mut countdown) = q_countdown.get_single_mut() {
        countdown.0.tick(time.delta());
        if countdown.0.just_finished() {
            if game.countdown == 3 {
                q_minimap.get_single_mut().unwrap().is_active = true;

                // undo Red Alert by
                // 1. setting to false and removing the flashing sprite
                game.red_alert = false;
                for ent in q_red_alert.iter() {
                    commands.entity(ent).despawn_recursive();
                }

                // 2. removing ships that were spawned during Red Alert
                // that are above the "normal limit"
                let level = &levels.0[game.level - 1];

                let mut i_count = 0;
                for _ in q_i_type.iter() {
                    i_count += 1;
                }

                let mut i_to_kill = level.start_i as isize - i_count as isize;
                for ent in q_i_type.iter() {
                    if i_to_kill < 1 {
                        break;
                    }
                    commands.entity(ent).despawn();
                    i_to_kill -= 1;
                }

                let mut p_count = 0;
                for _ in q_p_type.iter() {
                    p_count += 1;
                }

                let mut p_to_kill = level.start_p as isize - p_count as isize;
                for ent in q_p_type.iter() {
                    if p_to_kill < 1 {
                        break;
                    }
                    commands.entity(ent).despawn();
                    p_to_kill -= 1;
                }

                for mut text in &mut q_level_text {
                    text.sections[1].value = game.level.to_string();
                }

                commands.spawn((
                    SpriteBundle {
                        texture: game_assets.player.texture.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },
                    TextureAtlas::from(game_assets.player.layout.clone()),
                    Animation {
                        timer: Timer::from_seconds(0.35, TimerMode::Repeating),
                        n_sprites: 2,
                        one_time: false,
                    },
                    Player,
                    RigidBody::Dynamic,
                    Collider::ball(28.0),
                    Ccd::enabled(),
                    Sensor,
                    CollisionGroups::new(
                        Group::from_bits_truncate(0b00000001),
                        Group::from_bits_truncate(0b11001110),
                    ),
                    CameraOffset(Vec3::ZERO),
                    Velocity::default(),
                    RenderLayers::layer(0),
                    LevelNode,
                    GameNode,
                ));

                if game.setup {
                    game.setup = false;

                    let level = &levels.0[game.level - 1];

                    for star in &level.stars {
                        let marker = commands
                            .spawn((
                                ShapeBundle {
                                    spatial: SpatialBundle {
                                        transform: Transform {
                                            translation: world_to_minimap(Vec3::new(
                                                star.x, star.y, 3.0,
                                            )),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    path: GeometryBuilder::build_as(&shapes::Circle {
                                        radius: 7f32,
                                        center: Vec2::ZERO,
                                    }),
                                    ..default()
                                },
                                Fill::color(Color::srgb(0f32, 0.741, 0f32)),
                                MinimapStar,
                                RenderLayers::layer(1),
                                LevelNode,
                                GameNode,
                            ))
                            .id();

                        let star_config: (Handle<Image>, [(Vec3, Atlas); 6]) = if star.vert {
                            (
                                game_assets.v_star.clone(),
                                [
                                    (Vec3::new(48.0, 96.0, 1.0), game_assets.star_node_v1.clone()),
                                    (Vec3::new(112.0, 0.0, 1.0), game_assets.star_node_v2.clone()),
                                    (
                                        Vec3::new(48.0, -96.0, 1.0),
                                        game_assets.star_node_v3.clone(),
                                    ),
                                    (
                                        Vec3::new(-48.0, -96.0, 1.0),
                                        game_assets.star_node_v4.clone(),
                                    ),
                                    (
                                        Vec3::new(-112.0, 0.0, 1.0),
                                        game_assets.star_node_v5.clone(),
                                    ),
                                    (
                                        Vec3::new(-48.0, 96.0, 1.0),
                                        game_assets.star_node_v6.clone(),
                                    ),
                                ],
                            )
                        } else {
                            (
                                game_assets.h_star.clone(),
                                [
                                    (
                                        Vec3::new(96.0, -48.0, 1.0),
                                        game_assets.star_node_h1.clone(),
                                    ),
                                    (
                                        Vec3::new(0.0, -112.0, 1.0),
                                        game_assets.star_node_h2.clone(),
                                    ),
                                    (
                                        Vec3::new(-96.0, -48.0, 1.0),
                                        game_assets.star_node_h3.clone(),
                                    ),
                                    (
                                        Vec3::new(-96.0, 48.0, 1.0),
                                        game_assets.star_node_h4.clone(),
                                    ),
                                    (Vec3::new(0.0, 112.0, 1.0), game_assets.star_node_h5.clone()),
                                    (Vec3::new(96.0, 48.0, 1.0), game_assets.star_node_h6.clone()),
                                ],
                            )
                        };

                        commands
                            .spawn((
                                SpriteBundle {
                                    texture: star_config.0,
                                    transform: Transform::from_xyz(star.x, star.y, 1.0),
                                    ..default()
                                },
                                RigidBody::Fixed,
                                Collider::ball(20.0),
                                StarCore(marker),
                                CollisionGroups::new(
                                    Group::from_bits_truncate(0b00010000),
                                    Group::from_bits_truncate(0b00100000),
                                ),
                                Explodable(ExplodableType::StarCore),
                                RenderLayers::layer(0),
                                LevelNode,
                                GameNode,
                                Name::from("STAR"),
                            ))
                            .with_children(|parent| {
                                for (pos, atlas) in star_config.1 {
                                    parent
                                        .spawn((
                                            SpriteBundle {
                                                texture: atlas.texture.clone(),
                                                transform: Transform {
                                                    translation: pos,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            TextureAtlas {
                                                    layout: atlas.layout.clone(),
                                                    index: 0,
                                                },
                                            RigidBody::Fixed,
                                            Collider::ball(32.0),
                                            StarNode(Timer::from_seconds(
                                                thread_rng().gen_range(0.5..3.5),
                                                TimerMode::Once,
                                            )),
                                            Explodable(ExplodableType::StarNode),
                                            Sensor,
                                            CollisionGroups::new(
                                                Group::from_bits_truncate(0b0001000),
                                                Group::from_bits_truncate(0b0100001),
                                            ),
                                            RenderLayers::layer(0),
                                        ))
                                        .with_children(|node| {
                                            node.spawn(Collider::ball(300.0)).insert(Sensor);
                                        });
                                }
                            });
                    }

                    for rock in &level.rocks {
                        let texture = if rock.mine {
                            game_assets.mine.clone()
                        } else {
                            game_assets.asteroid.clone()
                        };

                        commands.spawn((
                            SpriteBundle {
                                texture,
                                transform: Transform {
                                    translation: Vec3::new(rock.x, rock.y, 10.0),
                                    ..default()
                                },
                                ..default()
                            },
                            RigidBody::Fixed,
                            Collider::ball(28.0),
                            Sensor,
                            CollisionGroups::new(
                                Group::from_bits_truncate(0b0000010),
                                Group::from_bits_truncate(0b1100101),
                            ),
                            Explodable(ExplodableType::Rock),
                            RenderLayers::layer(0),
                            LevelNode,
                            GameNode,
                        ));
                    }
                }

                if let Ok(mut cam_trans) = q_camera.get_single_mut() {
                    cam_trans.translation = Vec3::new(0.0, 0.0, cam_trans.translation.z);
                }

                life_events.send(UpdateLivesEvent);
            }

            if game.countdown <= 3 && game.countdown >= 1 {
                for ent in &q_countdown_text {
                    commands.entity(ent).despawn();
                }

                commands.spawn((
                    SpriteBundle {
                        texture: game_assets.countdown.texture.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                        ..default()
                    },
                    TextureAtlas {
                            layout: game_assets.countdown.layout.clone(),
                            index: game.countdown - 1,
                        },
                    CountdownText,
                    LevelNode,
                    GameNode,
                ));
            }

            if game.countdown == 0 {
                game.level_start_seconds = time.elapsed_seconds();
                game_state.set(ClassicGameState::Play);
                q_player.get_single_mut().unwrap().linvel = Vec2::new(0.0, 400.0);

                for ent in &q_countdown_text {
                    commands.entity(ent).despawn();
                }
            } else {
                game.countdown -= 1;
            }

            countdown.0.set_duration(Duration::from_secs_f32(1.0));
        }
    }
}

fn spawn_enemy_ships(
    mut commands: Commands,
    time: Res<Time>,
    levels: Res<Levels>,
    mut game: ResMut<Game>,
    game_assets: Res<GameAssets>,
    q_i_type: Query<Entity, (With<IType>, Without<PType>)>,
    q_p_type: Query<Entity, (With<PType>, Without<IType>)>,
    q_cam_offest: Query<&CameraOffset>,
) {
    // TODO maybe put current level in a Resource
    let level = &levels.0[game.level - 1];
    let mut rng = rand::thread_rng();
    let mut max_i = level.start_i;
    let mut max_p = level.start_p;

    game.itype_timer.tick(time.delta());
    game.ptype_timer.tick(time.delta());

    if game.red_alert {
        max_i = level.max_i;
        max_p = level.max_p;
    } else {
        if time.elapsed_seconds() - game.level_start_seconds > level.time_limit as f32 {
            game.red_alert = true;

            commands.spawn((
                SpriteBundle {
                    texture: game_assets.red_alert.texture.clone(),
                    transform: Transform::from_xyz(0.0, (750.0 / 2.0) - 20.0, 10.0),
                    ..default()
                },
                TextureAtlas::from(game_assets.red_alert.layout.clone()),
                Animation {
                    timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    n_sprites: 2,
                    one_time: false,
                },
                RenderLayers::layer(2),
                RedAlert,
                LevelNode,
                GameNode,
            ));
        }
    }

    let mut i_count = 0;
    for _ in q_i_type.iter() {
        i_count += 1;
    }

    let mut p_count = 0;
    for _ in q_p_type.iter() {
        p_count += 1;
    }

    if i_count < max_i && game.itype_timer.finished() {
        if let Ok(offset) = q_cam_offest.get_single() {
            let angle: f32 = rng.gen_range(-PI..PI);
            let trans = Vec3::new(angle.cos(), angle.sin(), 10.0)
                * Vec3::new(650.0 * 1.25, 650.0 * 1.25, 1.0)
                + offset.0;
            let angle: f32 = rng.gen_range(-PI..PI);

            commands.spawn((
                SpriteBundle {
                    texture: game_assets.i_type.clone(),
                    transform: Transform {
                        translation: trans,
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                },
                Velocity::default(),
                RigidBody::Dynamic,
                Collider::ball(26.0),
                Sensor,
                EnemyShip {
                    eneny_type: EnemyType::IType,
                    target: None,
                    time_got_target: None,
                    max_time_on_target: 0.25,
                    speed: 300.0,
                    turn_radius: 0.05,
                },
                CollisionGroups::new(
                    Group::from_bits_truncate(0b0000100),
                    Group::from_bits_truncate(0b1100111),
                ),
                Explodable(ExplodableType::IType),
                IType,
                RenderLayers::layer(0),
                LevelNode,
                GameNode,
            ));
        }
    }

    if p_count < max_p && game.ptype_timer.finished() {
        if let Ok(offset) = q_cam_offest.get_single() {
            let angle: f32 = rng.gen_range(-PI..PI);
            let trans = Vec3::new(angle.cos(), angle.sin(), 10.0)
                * Vec3::new(650.0 * 1.25, 650.0 * 1.25, 1.0)
                + offset.0;
            let angle: f32 = rng.gen_range(-PI..PI);

            commands.spawn((
                SpriteBundle {
                    texture: game_assets.p_type.clone(),
                    transform: Transform {
                        translation: trans,
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                },
                Velocity::default(),
                RigidBody::Dynamic,
                Collider::ball(26.0),
                Sensor,
                EnemyShip {
                    eneny_type: EnemyType::PType,
                    target: None,
                    time_got_target: None,
                    max_time_on_target: 3.0,
                    speed: 250.0,
                    turn_radius: 0.02,
                },
                CollisionGroups::new(
                    Group::from_bits_truncate(0b0000100),
                    Group::from_bits_truncate(0b1100111),
                ),
                Explodable(ExplodableType::PType),
                PType,
                RenderLayers::layer(0),
                LevelNode,
                GameNode,
            ));
        }
    }
}

fn listen_update_lives(
    mut commands: Commands,
    mut events: EventReader<UpdateLivesEvent>,
    game_assets: Res<GameAssets>,
    game: Res<Game>,
    q_lives: Query<Entity, With<Lives>>,
) {
    for _ in events.read() {
        for ent in q_lives.iter() {
            commands.entity(ent).despawn();
        }

        for i in 0..(game.lives - 1) {
            commands.spawn((
                SpriteBundle {
                    texture: game_assets.life.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            (1000.0 / 2.0) - 2.0 - (46.0 / 2.0) + (-48.0 * i as f32),
                            (750.0 / 2.0) - (46.0 / 2.0),
                            0.0,
                        ),
                        ..default()
                    },
                    ..default()
                },
                Lives,
                GameNode,
                RenderLayers::layer(2),
            ));
        }
    }
}

fn listen_player_death_classic(
    mut events: EventReader<PlayerDeathEvent>,
    mut game_state: ResMut<NextState<ClassicGameState>>,
    mut game: ResMut<Game>,
    mut q_mm_player: Query<&mut Transform, With<MinimapPlayer>>,
    mut minimap: Query<&mut Camera, With<MinimapCamera>>,
) {
    for _ in events.read() {
        println!("Event in listen_player_death_classic");
        game.lives -= 1;

        let mut cam = minimap.get_single_mut().unwrap();
        cam.is_active = false;
        for mut mm_trans in q_mm_player.iter_mut() {
            mm_trans.translation = world_to_minimap(Vec3::ZERO);
        }

        if game.lives > 0 {
            game.countdown = 4;
            game_state.set(ClassicGameState::Countdown);
        } else {
            game_state.set(ClassicGameState::GameOver);
        }
    }
}

fn star_update(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<ClassicGameState>>,
    mut level_events: EventWriter<SetupLevel>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    q_stars: Query<(Entity, &StarCore, &GlobalTransform, &Children)>,
    q_star_node: Query<&StarNode>,
    q_level_nodes: Query<Entity, With<LevelNode>>,
) {
    let mut star_count = 0;
    for (ent, star, trans, nodes) in q_stars.iter() {
        star_count += 1;
        let mut node_count = 0;
        for &n in nodes.iter() {
            if q_star_node.get(n).is_ok() {
                node_count += 1;
            }
        }

        if node_count == 0 {
            commands.entity(star.0).despawn();
            commands.entity(ent).despawn_recursive();

            explosion_events.send(ExplosionEvent {
                size: ExplosionSize::Big,
                x: trans.translation().x,
                y: trans.translation().y,
            });
        }
    }
    if star_count == 0 {
        for ent in &q_level_nodes {
            commands.entity(ent).despawn_recursive();
        }

        game.countdown = 4;
        game_state.set(ClassicGameState::Setup);
        level_events.send(SetupLevel);
    }
}

fn check_collisions(
    mut commands: Commands,
    mut game: ResMut<Game>,
    rapier_context: Res<RapierContext>,
    mut q_player: Query<(Entity, &GlobalTransform), With<Player>>,
    q_explodables: Query<(Entity, &GlobalTransform, &Explodable), With<Explodable>>,
    q_collidables: Query<(Entity, &GlobalTransform, &Collidable), With<Collidable>>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut player_death_events: EventWriter<PlayerDeathEvent>,
    q_stars: Query<(Entity, &StarCore)>,
    mut q_star_node_textures: Query<&mut TextureAtlas, With<StarNode>>,
) {
    // maybe not the best, if player is gone, do we still want explo-explo actions?
    if let Ok((player, p_trans)) = q_player.get_single_mut() {
        let mut p = true;

        for (e_ent, e_trans, explo) in q_explodables.iter() {
            // STEP 1 -- Explodable-Explodable interactions
            for (e_ent2, e_trans2, explo2) in q_explodables.iter() {
                // don't compare to self
                if e_ent == e_ent2 {
                    continue;
                }

                if rapier_context.intersection_pair(e_ent, e_ent2) == Some(true) {
                    for (ent, trans, exp) in [(e_ent, e_trans, explo), (e_ent2, e_trans2, explo2)] {
                        match exp.0 {
                            ExplodableType::StarNode => {
                                if let Ok(mut atlas) = q_star_node_textures.get_mut(e_ent) {
                                    atlas.index = 1;
                                }

                                // TODO need to match and update the collision groups (insert no work)
                                
                                commands
                                    .entity(ent)
                                    .insert(CollisionGroups::new(
                                        Group::from_bits_truncate(0b10000000),
                                        Group::from_bits_truncate(0b00100001),
                                    ))
                                    .insert(Collidable)
                                    .remove::<Explodable>()
                                    .remove::<StarNode>();

                                explosion_events.send(ExplosionEvent {
                                    size: ExplosionSize::Small,
                                    x: trans.translation().x,
                                    y: trans.translation().y,
                                });
                            }
                            ExplodableType::StarCore => {
                                for (star_ent, star) in q_stars.iter() {
                                    if star_ent == ent {
                                        commands.entity(star.0).despawn();
                                        break;
                                    }
                                }

                                commands.entity(ent).despawn_recursive();

                                explosion_events.send(ExplosionEvent {
                                    size: ExplosionSize::Big,
                                    x: trans.translation().x,
                                    y: trans.translation().y,
                                });
                            }
                            ExplodableType::Laser => {
                                commands.entity(ent).despawn();
                            }
                            ExplodableType::IType => {
                                explosion_events.send(ExplosionEvent {
                                    size: ExplosionSize::Small,
                                    x: trans.translation().x,
                                    y: trans.translation().y,
                                });
                                game.itype_timer.set_duration(Duration::from_secs_f32(2.0));
                                commands.entity(ent).despawn();
                            }
                            ExplodableType::PType => {
                                explosion_events.send(ExplosionEvent {
                                    size: ExplosionSize::Small,
                                    x: trans.translation().x,
                                    y: trans.translation().y,
                                });
                                game.ptype_timer.set_duration(Duration::from_secs_f32(2.0));
                                commands.entity(ent).despawn();
                            }
                            _ => {
                                explosion_events.send(ExplosionEvent {
                                    size: ExplosionSize::Small,
                                    x: trans.translation().x,
                                    y: trans.translation().y,
                                });
                                commands.entity(ent).despawn();
                            }
                        }
                    }
                }
            }

            // STEP 2 -- Player-Explodable interactions
            if rapier_context.intersection_pair(e_ent, player) == Some(true) {
                match explo.0 {
                    ExplodableType::StarNode => {
                        if let Ok(mut atlas) = q_star_node_textures.get_mut(e_ent) {
                            atlas.index = 1;
                        }
                    }
                    _ => commands.entity(e_ent).despawn(),
                }

                commands.entity(player).despawn();
                player_death_events.send(PlayerDeathEvent {});

                explosion_events.send(ExplosionEvent {
                    size: ExplosionSize::Small,
                    x: p_trans.translation().x,
                    y: p_trans.translation().y,
                });

                if explo.0 != ExplodableType::Laser {
                    explosion_events.send(ExplosionEvent {
                        size: ExplosionSize::Small,
                        x: e_trans.translation().x,
                        y: e_trans.translation().y,
                    });
                }
            }

            // STEP 3 -- Explodable-Collidable & Player-Collidable Interactions
            for (c_ent, _, __) in q_collidables.iter() {
                if rapier_context.intersection_pair(e_ent, c_ent) == Some(true) {
                    match explo.0 {
                        ExplodableType::Laser => {
                            commands.entity(e_ent).despawn();
                        }
                        _ => {}
                    }
                }

                if p && rapier_context.intersection_pair(c_ent, player) == Some(true) {
                    commands.entity(player).despawn();
                    player_death_events.send(PlayerDeathEvent {});
                    p = false;

                    explosion_events.send(ExplosionEvent {
                        size: ExplosionSize::Small,
                        x: p_trans.translation().x,
                        y: p_trans.translation().y,
                    });
                }
            }
        }
    }
}

/*
    * member
    . filter
    & member and filter

             8 7 6 5 4 3 2 1
    Player   . .     . . . *
     Rocks     . .     . * .
   Fighter     . .     & . .
  StarNode       .   *     .
  StarCore       . *
   P-Laser   .   * . . . .
   S-Laser     *       . . .
Collidable   *   .         .

*/
