use crate::player::control::KeyBindings;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct DelayedSystemTimer(Timer);

pub fn initial_grab_cursor(mut commands: Commands) {
    commands.insert_resource(DelayedSystemTimer(Timer::from_seconds(
        0.2,
        TimerMode::Once,
    )));
}

pub fn initial_grab_cursor_delayed(
    mut timer: ResMut<DelayedSystemTimer>,
    time: Res<Time>,
    mut has_run: Local<bool>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !*has_run {
        timer.tick(time.delta());
        if timer.finished() {
            if let Ok(mut window) = primary_window.get_single_mut() {
                toggle_grab_cursor(&mut window);
            } else {
                warn!("Primary window not found for `initial_grab_cursor`");
            }

            *has_run = true;
        }
    }
}

pub fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
        _ => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

pub fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    // app_exit_events: ResMut<Events<AppExit>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
            // app_exit_events.send(AppExit);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`");
    }
}
