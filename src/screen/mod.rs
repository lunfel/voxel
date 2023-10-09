use bevy::prelude::*;

pub struct ScreenPlugin;

fn toggle_fullscreen(

) {
    info!("todo screen");
}

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        info!("ScreenPlugin initializing");
        app.add_systems(Startup, toggle_fullscreen);
        info!("ScreenPlugin loaded");
    }
}