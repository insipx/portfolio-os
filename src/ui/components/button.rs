//! A Pretty Cool Button
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crate::primitives::ui::{FOLDER_ICON, FONT_REGULAR, NONE};

/// Marker Component for an OsIcon
#[derive(Component, Default)]
pub struct OsButton;

#[derive(Component, Default)]
pub struct OsIcon;

#[derive(Bundle, Default)]
pub struct CoolIcon {
    #[bundle]
    image: ImageBundle,
    icon: OsIcon,
}

impl CoolIcon {
    pub fn new(icon: Handle<Image>) -> Self {
        Self {
            image: ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                    ..default()
                },
                background_color: NONE.into(),
                image: UiImage {
                    texture: icon,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

/// A [`Button`] without a BackgroundColor
#[derive(Bundle, Default)]
pub struct OsButtonBundle {
    pub node: Node,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,
    pub os_icon: OsButton
}


/// spawns a container with a folder icon and some text
pub fn spawn_folder(commands: &mut ChildBuilder, font: &Handle<Font>, icon: &Handle<Image>, name: &str) {
    commands.spawn(OsButtonBundle {
        style: Style {
            size: Size::new(Val::Px(60.0), Val::Px(80.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            margin: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(2.0),
                top: Val::Px(2.0),
                bottom: Val::Px(2.0),
            },
            align_content: AlignContent::FlexStart,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(CoolIcon::new(icon.clone()));
        parent.spawn(TextBundle {
            text: Text::from_section(name, TextStyle {
                font: font.clone(),
                font_size: 12.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            }),
            style: Style {
                max_size: Size::new(Val::Px(60.0), Val::Undefined),
                align_content: AlignContent::FlexStart,
                ..default()
            },
            ..default()
        });
    });
}
