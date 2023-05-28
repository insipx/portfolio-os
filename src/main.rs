use bevy::{prelude::*, winit::WinitSettings, render::camera::RenderTarget, window::PrimaryWindow};
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
        (&Interaction, &mut Transform, &Children),
        (Changed<Interaction>, With<OsButton>),
    >,
    mut icon_query: Query<&mut BackgroundColor, With<OsIcon>>,
    // mut icon_query: Query<(&Interaction, &mut BackgroundColor), With<CoolIcon>>,
    // need to get window dimensions
    window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    // mut text_query: Query<&mut Text>,
) {

    let cursor_position = cursor_position(window, camera_q);
    for (interaction, mut transform, children) in &mut interaction_query {
        let mut icon_color = icon_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                println!("Transforming");
                println!("{:?}", cursor_position);
                *transform = Transform::from_translation(Vec3::new(cursor_position.x, cursor_position.y, 0.0));
                *icon_color = PRESSED_BUTTON.into();
                
            }
            Interaction::Hovered => {
                *icon_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *icon_color = NONE.into();
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
            spawn_folder(parent, &font, &folder_icon, "prawn");
            spawn_folder(parent, &font, &system_settings_icon, "Settings");
            spawn_folder(parent, &font, &system_preferences_display_icon, "Display");
            spawn_folder(parent, &font, &system_file_manager_icon, "Files");
            spawn_folder(parent, &font, &terminal_icon, "Terminal");
        });
}

// TODO: Figure out when this would return 0
/// Get the cursor position in the window
fn cursor_position(window: Query<&Window, With<PrimaryWindow>>, camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>) -> Vec2 {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();
    
    let Ok(window) = window.get_single() else {
        eprintln!("No window found!");
        return Vec2 { x: 0.0, y: 0.0 };
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        return world_position;
    } else {
        Vec2 { x: 0.0, y: 0.0 }
    }
}

