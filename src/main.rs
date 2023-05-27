use bevy::{prelude::*, winit::WinitSettings};
mod ui;
mod primitives;

use self::ui::prelude::*;
use self::primitives::ui::*;

fn main() {
    App::new()
        // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // adds all the plugins you'd expect in a game engine, like 2d/3d rendering, ui, asset
        // loading, windows, input etc. Importantly, it also adds an event loop
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(button_system)
        .run();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color) in &mut interaction_query {
        println!("Interaction: {:?}, {:?}", interaction, color);
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NONE.into();
            }
        }
    }
}

//UI: "AlignItems" is Up and Down when "Flex" is set to "Row"

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let folder_icon = asset_server.load(FOLDER_ICON);
    let font = asset_server.load(FONT_REGULAR);

    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                margin: UiRect{
                    left: Val::Percent(5.0),
                    right: Val::Percent(5.0),
                    top: Val::Percent(5.0),
                    bottom: Val::Percent(5.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_folder(parent, &font, &folder_icon, "CoolButton");
            spawn_folder(parent, &font, &folder_icon, "Another Cool Button");
        });
}
