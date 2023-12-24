use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use std::{f32::consts::PI, time::Duration};

use super::{
    game::{
        // systems
        animation,
        bullet_timer,
        button_system,
        destroy_game,
        follow_camera,
        listen_explosion,
        move_enemy_ships,
        player_input,
        star_node_shoot,
        update_minimap,
        world_to_minimap,

        Animation,
        CameraOffset,
        Collidable,
        Countdown,
        CountdownText,
        EnemyShip,
        EnemyType,
        Explodable,
        ExplodableType,
        ExplosionEvent,
        ExplosionSize,
        GameButton,
        GameButtonAction,
        GameCamera,
        GameNode,
        IType,
        LevelNode,
        MinimapCamera,
        MinimapPlayer,
        MinimapStar,
        Player,
        PlayerDeathEvent,
        PType,
        SetupLevel,
        StarCore,
        StarNode,
    },
    AppState, GameAssets,
};

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct StarSpawnTimer(Timer);

#[derive(Resource)]
struct Game {
    countdown: usize,
    setup: bool,
}

#[derive(Resource)]
struct GameStartSeconds(f32);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    None,
    Setup,
    Countdown,
    Play,
    GameOver,
}

pub struct EndlessPlugin;
impl Plugin for EndlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<ExplosionEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<SetupLevel>()
            .insert_resource(GameStartSeconds(0.0))
            .add_systems(OnEnter(AppState::Endless), setup_game)
            .add_systems(OnExit(AppState::Endless), destroy_game)
            .add_systems(OnEnter(GameState::GameOver), setup_gameover)
            .add_systems(Update, button_system.run_if(in_state(GameState::GameOver)))
            .add_systems(Update, countdown.run_if(in_state(GameState::Countdown)))
            .add_systems(
                Update,
                (
                    player_input,
                    follow_camera,
                    check_collisions,
                    bullet_timer,
                    spawn_ships_and_stars,
                    move_enemy_ships,
                    star_node_shoot,
                    star_update,
                    update_minimap,
                    update_score,
                )
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(
                Update,
                (animation, listen_player_death, listen_explosion)
                    .run_if(not(in_state(GameState::None))),
            )
            .add_systems(Update, setup_level.run_if(in_state(GameState::Setup)));
    }
}

fn setup_game(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_events: EventWriter<SetupLevel>,
) {
    rapier_config.gravity = Vec2::ZERO;

    // game camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            projection: OrthographicProjection {
                scale: 1.25,
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: true },
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
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        UiCameraConfig { show_ui: false },
        MinimapCamera,
        RenderLayers::from_layers(&[1]),
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
        Fill::color(Color::rgba(0f32, 0f32, 0f32, 0f32)),
        Stroke::new(Color::RED, 10.0),
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

    // game score
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "00:00",
            TextStyle {
                font: game_assets.font.clone(),
                font_size: 32.0,
                color: Color::WHITE,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        ScoreText,
        GameNode,
    ));

    // minimap player
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 5f32,
                center: Vec2::ZERO,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..default()
        },
        Fill::color(Color::WHITE),
        MinimapPlayer,
        RenderLayers::layer(1),
        GameNode,
    ));

    // Game resource
    commands.insert_resource(Game {
        countdown: 3,
        setup: false,
    });

    commands.spawn((
        Countdown(Timer::from_seconds(0.01, TimerMode::Repeating)),
        GameNode,
    ));

    commands.spawn((
        StarSpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)),
        GameNode,
    ));

    game_state.set(GameState::Setup);
    level_events.send(SetupLevel);
}

fn setup_gameover(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    q_camera: Query<&mut Transform, With<GameCamera>>,
) {
    let camera = q_camera.get_single().unwrap();
    let trans = camera.translation + Vec3::new(0.0, 100.0, -50.0);
    let texture = game_assets.game_over.clone();

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
                        background_color: Color::GRAY.into(),
                        ..default()
                    },
                    GameButton {
                        action: GameButtonAction::ReturnToMenu,
                        idle_color: Color::rgb(0.15, 0.15, 0.15),
                        hover_color: Color::rgb(0.25, 0.25, 0.25),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Menu",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn setup_level(
    mut level_event: EventReader<SetupLevel>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<GameState>>,
    mut minimap: Query<&mut Camera, With<MinimapCamera>>,
    mut q_mm_player: Query<&mut Transform, With<MinimapPlayer>>,
) {
    for _ in level_event.iter() {
        game.setup = true;

        let mut cam = minimap.get_single_mut().unwrap();
        cam.is_active = false;

        for mut mm_trans in q_mm_player.iter_mut() {
            mm_trans.translation = world_to_minimap(Vec3::ZERO);
        }

        game_state.set(GameState::Countdown);
    }
}

fn update_score(
    time: Res<Time>,
    game_start: Res<GameStartSeconds>,
    mut q_level_text: Query<&mut Text, With<ScoreText>>,
) {
    let seconds = (time.elapsed_seconds() - game_start.0) as usize;
    let score = format!("{:02}:{:02}", seconds / 60, seconds % 60);
    for mut text in &mut q_level_text {
        text.sections[0].value = score.to_string();
    }
}

fn countdown(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut q_countdown: Query<&mut Countdown>,
    mut q_player: Query<&mut Velocity, With<Player>>,
    q_countdown_text: Query<Entity, With<CountdownText>>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<NextState<GameState>>,
    mut q_camera: Query<&mut Transform, With<GameCamera>>,
    mut minimap: Query<&mut Camera, With<MinimapCamera>>,
    mut game_start: ResMut<GameStartSeconds>,
) {
    if let Ok(mut countdown) = q_countdown.get_single_mut() {
        countdown.0.tick(time.delta());
        if countdown.0.just_finished() {
            if game.countdown == 3 {
                minimap.get_single_mut().unwrap().is_active = true;

                // make player
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: game_assets.player.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 9.0),
                        ..default()
                    },
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

                    // TODO initial spawn of stars and rocks
                }

                if let Ok(mut cam_trans) = q_camera.get_single_mut() {
                    cam_trans.translation = Vec3::new(0.0, 0.0, cam_trans.translation.z);
                }

                // life_events.send(UpdateLivesEvent);
            }

            if game.countdown <= 3 && game.countdown >= 1 {
                for ent in &q_countdown_text {
                    commands.entity(ent).despawn();
                }

                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: game_assets.countdown.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                        sprite: TextureAtlasSprite {
                            index: game.countdown - 1,
                            ..default()
                        },
                        ..default()
                    },
                    CountdownText,
                    LevelNode,
                    GameNode,
                ));
            }

            if game.countdown == 0 {
                game_start.0 = time.elapsed_seconds();
                game_state.set(GameState::Play);
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

fn spawn_ships_and_stars(
    mut commands: Commands,
    game_start: Res<GameStartSeconds>,
    game_assets: Res<GameAssets>,
    q_fighter: Query<Entity, With<EnemyShip>>,
    q_stars: Query<Entity, With<StarCore>>,
    q_cam_offest: Query<&CameraOffset>,
    mut q_spawn_timer: Query<&mut StarSpawnTimer>,
    time: Res<Time>,
) {
    let seconds = (time.elapsed_seconds() - game_start.0) as usize;
    let max_fighters = ((seconds / 30) * 5) + 5;
    let max_stars = ((seconds / 30) * 3) + 3;

    let mut fighter_count = 0;
    for _ in q_fighter.iter() {
        fighter_count += 1;
    }

    if fighter_count < max_fighters {
        if let Ok(offset) = q_cam_offest.get_single() {
            let mut rng = rand::thread_rng();
            let angle: f32 = rng.gen_range(-PI..PI);
            let trans = Vec3::new(angle.cos(), angle.sin(), 10.0)
                * Vec3::new(850.0 * 1.25, 850.0 * 1.25, 1.0)
                + offset.0;
            let angle: f32 = rng.gen_range(-PI..PI);

            if rng.gen_bool(0.5) {
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
            } else {
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
                    Explodable(ExplodableType::Figher),
                    IType,
                    RenderLayers::layer(0),
                    LevelNode,
                    GameNode,
                ));
            }
        }
    }

    if let Ok(mut timer) = q_spawn_timer.get_single_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let mut star_count = 0;
            for _ in q_stars.iter() {
                star_count += 1;
            }

            let mut rng = rand::thread_rng();

            if star_count < max_stars {
                // TODO avoid overlaps
                // also maybe some buffer from the edge
                let x = rng.gen_range(-2400.0..=2400.0) as f32;
                let y = rng.gen_range(-2400.0..=2400.0) as f32;
                let vert = rng.gen_bool(0.5);

                let marker = commands
                    .spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shapes::Circle {
                                radius: 7f32,
                                center: Vec2::ZERO,
                            }),
                            transform: Transform {
                                translation: world_to_minimap(Vec3::new(x, y, 3.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Fill::color(Color::hex("00bd00").unwrap()),
                        MinimapStar,
                        RenderLayers::layer(1),
                        LevelNode,
                        GameNode,
                    ))
                    .id();

                let star_config: (Handle<Image>, [(Vec3, Handle<TextureAtlas>); 6]) = if vert {
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
                            transform: Transform::from_xyz(x, y, 1.0),
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
                        for (pos, texture) in star_config.1 {
                            parent
                                .spawn((
                                    SpriteSheetBundle {
                                        texture_atlas: texture,
                                        transform: Transform {
                                            translation: pos,
                                            ..default()
                                        },
                                        ..default()
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

            timer.0.set_duration(Duration::from_secs_f32(3.0));
        }
    }
}

fn listen_player_death(
    mut events: EventReader<PlayerDeathEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut q_mm_player: Query<&mut Transform, With<MinimapPlayer>>,
    mut minimap: Query<&mut Camera, With<MinimapCamera>>,
) {
    for _ in events.iter() {
        let mut cam = minimap.get_single_mut().unwrap();
        cam.is_active = false;

        for mut mm_trans in q_mm_player.iter_mut() {
            mm_trans.translation = world_to_minimap(Vec3::ZERO);
        }

        game_state.set(GameState::GameOver);
    }
}

fn star_update(
    mut commands: Commands,
    q_stars: Query<(Entity, &StarCore, &GlobalTransform, &Children)>,
    q_star_node: Query<&StarNode>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut q_star_timer: Query<&mut StarSpawnTimer>,
) {
    for (ent, star, trans, nodes) in q_stars.iter() {
        let mut node_count = 0;
        for &n in nodes.iter() {
            if q_star_node.get(n).is_ok() {
                node_count += 1;
            }
        }

        if node_count == 0 {
            if let Ok(mut timer) = q_star_timer.get_single_mut() {
                if timer.0.finished() {
                    timer.0.set_duration(Duration::from_secs_f32(3.0));
                }
            }

            commands.entity(star.0).despawn();
            commands.entity(ent).despawn_recursive();

            explosion_events.send(ExplosionEvent {
                size: ExplosionSize::Big,
                x: trans.translation().x,
                y: trans.translation().y,
            });
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut q_player: Query<(Entity, &GlobalTransform), With<Player>>,
    q_explodables: Query<(Entity, &GlobalTransform, &Explodable), With<Explodable>>,
    q_collidables: Query<(Entity, &GlobalTransform, &Collidable), With<Collidable>>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut player_death_events: EventWriter<PlayerDeathEvent>,
    q_stars: Query<(Entity, &StarCore)>,
    mut q_star_timer: Query<&mut StarSpawnTimer>,
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
                                commands
                                    .entity(ent)
                                    .insert(TextureAtlasSprite {
                                        index: 1,
                                        ..default()
                                    })
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

                                if let Ok(mut timer) = q_star_timer.get_single_mut() {
                                    if timer.0.finished() {
                                        timer.0.set_duration(Duration::from_secs_f32(3.0));
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
                        commands.entity(e_ent).insert(TextureAtlasSprite {
                            index: 1,
                            ..default()
                        });
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

fighter dropping mines?

seconds  max_fighters  max_stars
   0-30             5          3
  30-60            10          6
  60-90            15          9
    etc

  ((sec // 30) * 5) + 5
  ((sec // 30) * 3) + 3
*/
