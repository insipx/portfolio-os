use bevy::{prelude::*, winit::WinitSettings};
mod primitives;
mod systems;
mod ui;

use self::primitives::ui::*;
use self::systems::*;
use self::ui::prelude::*;

fn main() {
    App::new()
        // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // adds all the plugins you'd expect in a game engine, like 2d/3d rendering, ui, asset
        // loading, windows, input etc. Importantly, it also adds an event loop
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(button_system)
        .add_system(drag_and_drop_system)
        .run();
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<OsButton>)>,
    mut icon_query: Query<&mut BackgroundColor, With<OsIcon>>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut icon_color = icon_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *icon_color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *icon_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *icon_color = NORMAL_BUTTON.into();
            }
        }
    }
}

//UI: "AlignItems" is Up and Down when "Flex" is set to "Row"
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let folder_icon = asset_server.load(FOLDER_ICON);
    let system_settings_icon = asset_server.load(SYSTEM_SETTINGS_ICON);
    let system_preferences_display_icon = asset_server.load(SYSTEM_PREFERENCES_DISPLAY_ICON);
    let system_file_manager_icon = asset_server.load(SYSTEM_FILE_MANAGER_ICON);
    let terminal_icon = asset_server.load(TERMINAL_ICON);

    let font = asset_server.load(FONT_REGULAR);
    let wallpaper = asset_server.load(WALLPAPER_WINDOWS95);

    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect {
                    left: Val::Percent(1.0),
                    right: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                ..default()
            },
            image: UiImage {
                texture: wallpaper,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_folder(parent, &font, &folder_icon, "yuh");
            spawn_folder(parent, &font, &system_settings_icon, "Settings");
            spawn_folder(parent, &font, &system_preferences_display_icon, "Display");
            spawn_folder(parent, &font, &system_file_manager_icon, "Files");
            spawn_folder(parent, &font, &terminal_icon, "Terminal");
        });
}
