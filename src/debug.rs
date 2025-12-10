use bevy::prelude::*;

use super::{AppState, GameAssets};

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct MenuButton {
    action: MenuButtonAction,
    idle_color: Color,
    hover_color: Color,
}

enum MenuButtonAction {
    Quit,
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Debug), setup)
            .add_systems(Update, button_system.run_if(in_state(AppState::Debug)))
            .add_systems(OnExit(AppState::Debug), despawn_debug);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_assets: Res<GameAssets>) {
    commands.spawn((Camera2dBundle::default(), Menu));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("i_type.png"),
            ..default()
        },
        Menu,
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
            Menu,
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
                        background_color: Color::rgb(0.86, 0.88, 0.91).into(),
                        ..default()
                    },
                    MenuButton {
                        action: MenuButtonAction::Quit,
                        idle_color: Color::rgb(0.86, 0.88, 0.91),
                        hover_color: Color::rgb(0.58, 0.60, 0.69),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Back to Menu",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 30.0,
                            color: Color::rgb(0.0, 0.0, 0.0),
                        },
                    ));
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &MenuButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<AppState>>,
) {
    // println!("Button System");
    for (interaction, button, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button.action {
                MenuButtonAction::Quit => game_state.set(AppState::Menu),
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

fn despawn_debug(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    println!("Byebye Debug");
    for ent in &menu {
        println!("ent!");
        commands.entity(ent).despawn();
    }
}
