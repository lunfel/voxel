pub mod control;

use bevy::prelude::*;

use crate::{
    player::control::{
        cursor_grab, follow_player_look_left_right, initial_grab_cursor, player_look, player_move,
        setup_player, InputState, JumpTimer, KeyBindings, MovementSettings,
    }
};
use crate::player::control::{follow_player_look_up_down, follow_player_position};
use crate::world::world_generation::generate_world;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("PlayerPlugin initializing");
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .init_resource::<JumpTimer>()
            .add_systems(Startup, setup_player.after(generate_world))
            .add_systems(Startup, initial_grab_cursor.after(generate_world))
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, follow_player_look_left_right)
            .add_systems(Update, follow_player_look_up_down)
            .add_systems(Update, follow_player_position)
            .add_systems(Update, cursor_grab);

        info!("PlayerPlugin loaded");
    }
}