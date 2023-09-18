use bevy::prelude::*;

pub struct ScreenPlugin;

fn toggle_fullscreen(

) {
    info!("todo screen");
}

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, toggle_fullscreen);
    }
}