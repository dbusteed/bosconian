use super::{AppState, GameAssets};
use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::Duration,
};

const TWO_PI: f32 = 2.0 * PI;

//
// enums
//

pub enum EnemyType {
    IType,
    PType,
}

pub enum ExplosionSize {
    Small,
    Big,
}

#[derive(PartialEq)]
pub enum ExplodableType {
    Rock,
    Figher,
    StarCore,
    StarNode,
    Laser,
    IType,
    PType,
}

pub enum GameButtonAction {
    ReturnToMenu,
}

//
// components
//
#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub n_sprites: usize,
    pub one_time: bool,
}

#[derive(Component)]
pub struct CameraOffset(pub Vec3);

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct Countdown(pub Timer);

#[derive(Component)]
pub struct CountdownText;

#[derive(Component)]
pub struct EnemyShip {
    pub eneny_type: EnemyType,
    pub target: Option<Vec2>,
    pub time_got_target: Option<f32>,
    pub max_time_on_target: f32,
    pub speed: f32,
    pub turn_radius: f32,
}

#[derive(Component)]
pub struct Explodable(pub ExplodableType);

#[derive(Component)]
pub struct GameButton {
    pub action: GameButtonAction,
    pub idle_color: Color,
    pub hover_color: Color,
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct GameNode;

#[derive(Component)]
pub struct IType;

#[derive(Component)]
pub struct PType;

#[derive(Component)]
pub struct LevelNode;

#[derive(Component)]
pub struct MinimapCamera;

#[derive(Component)]
pub struct MinimapPlayer;

#[derive(Component)]
pub struct MinimapStar;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Component)]
pub struct Projectile(pub Timer);

#[derive(Component)]
pub struct StarCore(pub Entity);

#[derive(Component)]
pub struct StarNode(pub Timer);

//
// events
//
#[derive(Event)]
pub struct ExplosionEvent {
    pub size: ExplosionSize,
    pub x: f32,
    pub y: f32,
}

#[derive(Event)]
pub struct PlayerDeathEvent;

#[derive(Event)]
pub struct SetupLevel;

//
// systems
//
pub fn animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Animation, &mut TextureAtlasSprite)>,
) {
    for (entity, mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            sprite.index += 1;
            if sprite.index >= anim.n_sprites {
                if anim.one_time {
                    commands.entity(entity).despawn();
                } else {
                    sprite.index = 0;
                }
            }
        }
    }
}

pub fn bullet_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut q_bullets: Query<(Entity, &Transform, &mut Projectile)>,
    q_camera: Query<&Transform, With<CameraOffset>>,
) {
    //
    // despawn bullets after a certain number of seconds,
    // or if they go off screen
    //

    for cam_trans in q_camera.iter() {
        for (entity, trans, mut projectile) in q_bullets.iter_mut() {
            projectile.0.tick(time.delta());

            // this isn't perfect since we're only looking in one direction but it's okay
            let diff = trans
                .translation
                .truncate()
                .distance(cam_trans.translation.truncate());
            if diff > 750.0 || projectile.0.just_finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &GameButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button.action {
                GameButtonAction::ReturnToMenu => app_state.set(AppState::Menu),
            },
            Interaction::Hovered => {
                *color = button.hover_color.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = button.idle_color.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn destroy_game(mut commands: Commands, menu: Query<Entity, With<GameNode>>) {
    for ent in &menu {
        commands.entity(ent).despawn_recursive();
    }
}

pub fn follow_camera(
    mut q_camera: Query<&mut Transform, With<GameCamera>>,
    q_player: Query<&CameraOffset>,
) {
    for mut cam_trans in q_camera.iter_mut() {
        for offset in q_player.iter() {
            cam_trans.translation = Vec3::new(offset.0.x, offset.0.y, cam_trans.translation.z);
        }
    }
}

pub fn listen_explosion(
    mut commands: Commands,
    mut events: EventReader<ExplosionEvent>,
    game_assets: Res<GameAssets>,
) {
    for evt in events.iter() {
        match evt.size {
            ExplosionSize::Small => {
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: game_assets.explosion.clone(),
                        transform: Transform::from_xyz(evt.x, evt.y, 3.0),
                        ..default()
                    },
                    Animation {
                        timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                        n_sprites: 3,
                        one_time: true,
                    },
                    GameNode,
                ));
            }
            ExplosionSize::Big => {
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: game_assets.big_explosion.clone(),
                        transform: Transform::from_xyz(evt.x, evt.y, 3.0),
                        ..default()
                    },
                    Animation {
                        timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                        n_sprites: 3,
                        one_time: true,
                    },
                    GameNode,
                ));
            }
        }
    }
}

pub fn move_enemy_ships(
    mut query: Query<(&mut Velocity, &mut Transform, &mut EnemyShip)>,
    q_player: Query<&CameraOffset>,
    time: Res<Time>,
) {
    if let Ok(player_pos) = q_player.get_single() {
        // TODO can they avoid rocks to some degree?
        for (mut vel, mut trans, mut ship) in query.iter_mut() {
            if let Some(target) = ship.target {
                let angle = f32::atan2(
                    target.y - trans.translation.y,
                    target.x - trans.translation.x,
                );

                let (axis, mut rot) = trans.rotation.to_axis_angle();
                rot = (axis * rot).z;

                // does it need to rotate?
                if (angle - rot).abs() > 0.05 {
                    let a = (angle + TWO_PI) % (TWO_PI);
                    let r = (rot + TWO_PI) % (TWO_PI);

                    let mut diff = a - r;
                    if diff.abs() > PI {
                        diff += TWO_PI;
                    }

                    // which way to rotate?
                    if diff.is_sign_positive() {
                        trans.rotate_z(ship.turn_radius);
                    } else {
                        trans.rotate_z(-ship.turn_radius);
                    }
                }

                if trans.translation.truncate().distance(target) < 10.0 {
                    ship.target = None;
                } else {
                    vel.linvel = Vec2::from_angle(rot) * ship.speed;
                }

                // find a new target if it's searched to long
                if let Some(target_time) = ship.time_got_target {
                    if time.elapsed_seconds() - target_time > 0.5 {
                        ship.target = None;
                        ship.time_got_target = None;
                    }
                }
            } else {
                let mut rng = rand::thread_rng();
                let offset = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
                ship.target = Some(player_pos.0.truncate() + offset);
                ship.time_got_target = Some(time.elapsed_seconds());
            }
        }
    }
}

pub fn player_input(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_assets: Res<GameAssets>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut CameraOffset), With<Player>>,
) {
    if let Ok((mut trans, mut vel, mut offset)) = player.get_single_mut() {
        if let Some(keycode) = kb.get_just_pressed().last() {
            let mut rot: f32 = -1.0;
            match keycode {
                KeyCode::W => {
                    vel.linvel = Vec2::new(0.0, 400.0);
                    rot = 0.0;
                }
                KeyCode::A => {
                    vel.linvel = Vec2::new(-400.0, 0.0);
                    rot = FRAC_PI_2;
                }
                KeyCode::S => {
                    vel.linvel = Vec2::new(0.0, -400.0);
                    rot = PI;
                }
                KeyCode::D => {
                    vel.linvel = Vec2::new(400.0, 0.0);
                    rot = 3.0 * FRAC_PI_2;
                }

                KeyCode::Return => {
                    let texture1: Handle<Image>;
                    let texture2: Handle<Image>;
                    if vel.linvel.abs().x > 0f32 {
                        texture1 = game_assets.h_laser.clone();
                        texture2 = game_assets.h_laser.clone();
                    } else {
                        texture1 = game_assets.v_laser.clone();
                        texture2 = game_assets.v_laser.clone();
                    }

                    commands.spawn(AudioBundle {
                        source: game_assets.laser_sound.clone(),
                        settings: PlaybackSettings {
                            volume: Volume::Relative(VolumeLevel::new(0.25)),
                            ..default()
                        },
                        ..default()
                    });

                    commands.spawn((
                        SpriteBundle {
                            texture: texture1,
                            transform: Transform::from_xyz(
                                trans.translation.x,
                                trans.translation.y,
                                1.0,
                            ),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Ccd::enabled(),
                        Collider::ball(5.0),
                        Sensor,
                        Explodable(ExplodableType::Laser),
                        PlayerProjectile,
                        Projectile(Timer::from_seconds(5.0, TimerMode::Once)),
                        CollisionGroups::new(
                            Group::from_bits_truncate(0b00100000),
                            Group::from_bits_truncate(0b10011110),
                        ),
                        Velocity {
                            linvel: vel.linvel * 3.0,
                            ..default()
                        },
                        LevelNode,
                        GameNode,
                    ));

                    commands.spawn((
                        SpriteBundle {
                            texture: texture2,
                            transform: Transform::from_xyz(
                                trans.translation.x,
                                trans.translation.y,
                                1.0,
                            ),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Ccd::enabled(),
                        Collider::ball(5.0),
                        Sensor,
                        Explodable(ExplodableType::Laser),
                        PlayerProjectile,
                        Projectile(Timer::from_seconds(5.0, TimerMode::Once)),
                        CollisionGroups::new(
                            Group::from_bits_truncate(0b00100000),
                            Group::from_bits_truncate(0b10011110),
                        ),
                        Velocity {
                            linvel: vel.linvel * -2.5,
                            ..default()
                        },
                        LevelNode,
                        GameNode,
                    ));
                }
                _ => {}
            }
            if rot >= 0.0 {
                trans.rotation = Quat::from_rotation_z(rot);
            }
        }

        offset.0 = trans.translation;
    }
}

pub fn star_node_shoot(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut q_nodes: Query<(&GlobalTransform, &mut StarNode, &Children)>,
    mut q_player: Query<(Entity, &GlobalTransform), With<Player>>,
    game_assets: Res<GameAssets>,
) {
    if let Ok((p_ent, p_trans)) = q_player.get_single_mut() {
        for (trans, mut node, children) in q_nodes.iter_mut() {
            for child in children.iter() {
                if rapier_context.intersection_pair(*child, p_ent) == Some(true) {
                    node.0.tick(time.delta());
                    if node.0.finished() {
                        let mut rng = rand::thread_rng();
                        node.0
                            .set_duration(Duration::from_secs_f32(rng.gen_range(1.0..4.0)));
                        node.0.reset();

                        let vel = (p_trans.translation().truncate()
                            - trans.translation().truncate())
                        .normalize()
                            * 150.0;

                        commands.spawn((
                            SpriteSheetBundle {
                                texture_atlas: game_assets.star_node_laser.clone(),
                                transform: Transform::from_xyz(
                                    trans.translation().x,
                                    trans.translation().y,
                                    5.0,
                                ),
                                ..default()
                            },
                            Animation {
                                timer: Timer::from_seconds(0.08, TimerMode::Repeating),
                                n_sprites: 4,
                                one_time: false,
                            },
                            RigidBody::Dynamic,
                            Ccd::enabled(),
                            Collider::ball(5.0),
                            Sensor,
                            Explodable(ExplodableType::Laser),
                            Projectile(Timer::from_seconds(5.0, TimerMode::Once)),
                            CollisionGroups::new(
                                Group::from_bits_truncate(0b1000000),
                                Group::from_bits_truncate(0b0000111),
                            ),
                            Velocity {
                                linvel: vel,
                                ..default()
                            },
                            LevelNode,
                            GameNode,
                        ));
                    }
                }
            }
        }
    }
}

pub fn world_to_minimap(world_pos: Vec3) -> Vec3 {
    world_pos / Vec3::new(20.0, 20.0, 1.0)
}

pub fn update_minimap(
    q_player: Query<&CameraOffset>,
    mut q_mm_player: Query<&mut Transform, With<MinimapPlayer>>,
) {
    for trans in q_player.iter() {
        for mut mm_trans in q_mm_player.iter_mut() {
            mm_trans.translation = world_to_minimap(trans.0);
        }
    }
}
