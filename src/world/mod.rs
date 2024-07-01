use bevy::app::App;
use bevy::prelude::*;

use systems::chunk::DebugColliderTimer;
use game_world::{bind_fresh_game_chunk_entity_to_game_world, GameWorld};

pub mod block;
pub mod chunk;
pub mod systems;
pub mod game_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        info!("WorldPlugin initializing");
        app
            .insert_resource(DebugColliderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .init_resource::<GameWorld>()
            .add_systems(Update, bind_fresh_game_chunk_entity_to_game_world);
            // .add_systems(Update, debug_collider_counts);
            // .add_systems(Update, render_dirty_chunk)
            // .add_systems(Update, enable_close_colliders)
            // .add_systems(Update, disable_far_colliders);
        info!("WorldPlugin loaded");
    }
}