//! A Pretty Cool Button
use bevy::prelude::*;
use crate::primitives::ui::{FOLDER_ICON, FONT_REGULAR};

#[derive(Bundle)]
pub struct CoolButton {
    #[bundle]
    button: ButtonBundle
}

impl CoolButton {
    pub fn new(icon: Handle<Image>) -> Self {
        Self {
            button: ButtonBundle {
                style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                        ..default()
                    },
                    // background_color: NORMAL_BUTTON.into(),
                    image: UiImage {
                        texture: icon,
                        ..default()
                    },
                    ..default()
            }
        }
    }
}


/// spawns a container with a folder icon and some text
pub fn spawn_folder(commands: &mut ChildBuilder, font: &Handle<Font>, icon: &Handle<Image>, name: &str) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(48.0), Val::Px(48.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(CoolButton::new(icon.clone()));
        parent.spawn(TextBundle::from_section(
            name,
            TextStyle {
                font: font.clone(),
                font_size: 15.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            }).with_style(Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                ..default()
            }));
    });
}
