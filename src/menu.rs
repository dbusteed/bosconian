use bevy::{app::AppExit, prelude::*};
use webbrowser;

use super::{AppState, GameAssets};

const REPO_URL: &str = "https://github.com/dbusteed/bosconian";

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct MenuButton {
    action: MenuButtonAction,
    idle_color: Color,
    hover_color: Color,
}

enum MenuButtonAction {
    VisitRepo,
    Classic,
    Endless,
    Quit,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(Update, button_system.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_menu);
    }
}

fn setup_menu(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d, Menu, Name::from("Menu Camera")));

    // background
    commands.spawn((
        Sprite {
            image: game_assets.menu_background.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Menu,
        Name::from("Menu Background"),
    ));

    // github button
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                ..default()
            },
            BackgroundColor::from(Color::BLACK),
            MenuButton {
                action: MenuButtonAction::VisitRepo,
                idle_color: Color::srgb(0.58, 0.60, 0.69),
                hover_color: Color::srgb(1.0, 1.0, 1.0),
            },
            Menu,
            Name::from("Menu GitHub"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: asset_server.load("github.png"),
                    color: Color::srgb(0.58, 0.60, 0.69),
                    ..default()
                },
            ));
        });

    // menu buttons
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(400.0),
                width: Val::Px(1000.0),
                height: Val::Px(350.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Menu,
            Name::from("Menu Buttons"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(225.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(Color::srgb(0.86, 0.88, 0.91)),
                    MenuButton {
                        action: MenuButtonAction::Classic,
                        idle_color: Color::srgb(0.86, 0.88, 0.91),
                        hover_color: Color::srgb(0.58, 0.60, 0.69),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Classic"),
                        TextColor(Color::srgb(0.0, 0.0, 0.0)),
                        TextFont {
                            font: game_assets.font.clone(),
                            font_size: 26.0,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(225.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(Color::srgb(0.86, 0.88, 0.91)),
                    MenuButton {
                        action: MenuButtonAction::Endless,
                        idle_color: Color::srgb(0.86, 0.88, 0.91),
                        hover_color: Color::srgb(0.58, 0.60, 0.69),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Endless"),
                        TextColor(Color::srgb(0.0, 0.0, 0.0)),
                        TextFont {
                            font: game_assets.font.clone(),
                            font_size: 26.0,
                            ..default()
                        },
                    ));
                });

            // no quit button on WASM
            if !cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(225.0),
                            height: Val::Px(70.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderRadius::all(Val::Px(10.0)),
                        BackgroundColor(Color::srgb(0.86, 0.88, 0.91)),
                        MenuButton {
                            action: MenuButtonAction::Quit,
                            idle_color: Color::srgb(0.86, 0.88, 0.91),
                            hover_color: Color::srgb(0.58, 0.60, 0.69),
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Quit"),
                            TextColor(Color::srgb(0.0, 0.0, 0.0)),
                            TextFont {
                                font: game_assets.font.clone(),
                                font_size: 26.0,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &MenuButton,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&Children>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut q_image_nodes: Query<&mut ImageNode>,
    mut game_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    // println!("Button System");
    for (interaction, button, mut color, mut border_color, opt_children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button.action {
                    MenuButtonAction::Classic => game_state.set(AppState::Classic),
                    MenuButtonAction::Endless => game_state.set(AppState::Endless),
                    // .write returns the eventID, suppress with ;
                    MenuButtonAction::Quit => {
                        exit.write(AppExit::Success);
                    }
                    MenuButtonAction::VisitRepo => {
                        match webbrowser::open(REPO_URL) {
                            Ok(()) => {} // do nothing. if matched, the command was executed successfully
                            Err(_) => println!(
                                "Unable to open browser, check out the code at {}",
                                REPO_URL
                            ),
                        }
                    }
                }
            }
            Interaction::Hovered => match button.action {
                MenuButtonAction::VisitRepo => {
                    if let Some(children) = opt_children {
                        for child in children.iter() {
                            if let Ok(mut image_node) = q_image_nodes.get_mut(child) {
                                image_node.color = button.hover_color.into();
                            }
                        }
                    }
                }
                _ => {
                    *color = button.hover_color.into();
                    border_color.bottom = Color::WHITE;
                }
            },
            Interaction::None => match button.action {
                MenuButtonAction::VisitRepo => {
                    if let Some(children) = opt_children {
                        for child in children.iter() {
                            if let Ok(mut image_node) = q_image_nodes.get_mut(child) {
                                image_node.color = button.idle_color.into();
                            }
                        }
                    }
                }
                _ => {
                    *color = button.idle_color.into();
                    border_color.bottom = Color::BLACK;
                }
            },
        }
    }
}

fn despawn_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for ent in &menu {
        commands.entity(ent).despawn();
    }
}
