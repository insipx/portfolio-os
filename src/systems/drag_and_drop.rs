//! A system for dragging and dropping OsButtons within their parent containers.
// TODO: Fix being allowed to drag icons off-screen.
// TODO: Add delay to when dragging begins and/or check if cursor has moved at all since click (add
// TODO: Because icons are organized by flexbox, they automatically re-arrange themselves...
// states to [`ClickedState`])

use crate::ui::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};


/// System allowing [`OsButton`]'s to be dragged-and-dropped
pub fn drag_and_drop_system(
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut os_buttons: Query<
        (
            &mut Style,
            &mut ClickedState,
            &Interaction,
            &mut ButtonTimer,
        ),
        (
            With<OsButton>,
            Option<Changed<Interaction>>,
            Option<Changed<ClickedState>>,
            With<ButtonTimer>,
        ),
    >,
    mouse_button: Res<Input<MouseButton>>,
) {
    let Ok(window) = window.get_single() else {
        eprintln!("No window found!");
        return;
    };
    let cursor_position = cursor_position(window, camera_q);
    let ui_position = cursor_position_to_ui_rect(window, cursor_position);

    if mouse_button.just_pressed(MouseButton::Left) {
        for (mut style, mut clicked_state, interaction, mut timer) in os_buttons.iter_mut() {
            if interaction == &Interaction::Clicked {
                *style = Style {
                    position_type: PositionType::Absolute,
                    position: ui_position,
                    ..*style
                };
                *clicked_state = ClickedState::Dragging;
                timer.timer.reset();
            }
        }
    }
    if mouse_button.pressed(MouseButton::Left) {
        for (mut style, mut clicked_state, _, mut timer) in os_buttons.iter_mut() {
            if *clicked_state == ClickedState::Dragging {
                *style = Style {
                    position_type: PositionType::Absolute,
                    position: ui_position,
                    ..*style
                };
                // make sure to set the state to keep `Changed` filter working.
                *clicked_state = ClickedState::Dragging;
                timer.timer.tick(time.delta());
                println!("Dragging for {:?}", timer.elapsed());
            }
        }
    }
    if mouse_button.just_released(MouseButton::Left) {
        for (mut style, mut clicked_state, _, mut timer) in os_buttons.iter_mut() {
            if *clicked_state == ClickedState::Dragging {
                *style = Style {
                    position_type: PositionType::Absolute,
                    position: ui_position,
                    ..*style
                };
                *clicked_state = ClickedState::Idle;
                timer.timer.reset();
            }
        }
    }
}

/// Get the cursor position in the [`Window`]
fn cursor_position(
    window: &Window,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
) -> Vec2 {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    // println!("{:?}", window.cursor_position());
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        return world_position;
    } else {
        // cursor is outside the window
        Vec2 { x: 0.0, y: 0.0 }
    }
}

/// Convert a cursor position to a [`UiRect`] position
fn cursor_position_to_ui_rect(window: &Window, cursor_position: Vec2) -> UiRect {
    let (screen_height, screen_width) = (window.height(), window.width());
    let new_x = cursor_position.x + (screen_width) / 2f32;
    let new_y = (screen_height / 2f32) - cursor_position.y;

    UiRect {
        left: Val::Px(new_x),
        right: Val::Px(new_x),
        top: Val::Px(new_y),
        bottom: Val::Px(new_y),
    }
}
