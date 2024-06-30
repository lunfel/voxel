use std::fmt::Formatter;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode, WindowResolution};
use bevy::window::WindowMode::{BorderlessFullscreen, Fullscreen};
use crate::systems::player::player_control::KeyBindings;

pub struct ScreenPlugin;

#[derive(Resource, Deref, DerefMut)]
pub struct PreviousWindowMode(WindowMode);

impl Default for PreviousWindowMode {
    fn default() -> Self {
        PreviousWindowMode(WindowMode::BorderlessFullscreen)
    }
}

fn toggle_fullscreen(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut window: Query<(&mut Window, Entity)>,
    mut focused_event: EventWriter<CursorMoved>,
) {
    if let Ok((mut window, entity)) = window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_fullscreen) {
            window.mode = Fullscreen;

            let width = window.width();
            let height = window.height();
            let position = Vec2::new(width / 2.0, height / 2.0);
            window.set_cursor_position(Some(position));
            focused_event.send(CursorMoved {
                window: entity,
                position,
            });
        } else if keys.just_pressed(KeyCode::G) {
            window.mode = WindowMode::Windowed;

            let width = window.width();
            let height = window.height();
            let position = Vec2::new(width / 2.0, height / 2.0);
            window.set_cursor_position(Some(position));
            focused_event.send(CursorMoved {
                window: entity,
                position,
            });
        }
    }
}

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        info!("ScreenPlugin initializing");
        app.init_resource::<PreviousWindowMode>();
        app.add_systems(Update, toggle_fullscreen);
        info!("ScreenPlugin loaded");
    }
}