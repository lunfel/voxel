use bevy::app::App;
use bevy::prelude::*;

use systems::chunk::DebugColliderTimer;
use game_world::{bind_fresh_game_chunk_entity_to_game_world, GameWorld};
use crate::world::generate_as_we_go::{check_for_player_chunk_position_update, spawn_and_destroy_chunks, update_player_last_chunk_coord, PlayerChangedChunkCoordEvent, PlayerLastChunkCoord};

pub mod block;
pub mod chunk;
pub mod systems;
pub mod game_world;
pub mod world_generation;
mod generate_as_we_go;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        info!("WorldPlugin initializing");
        app
            .insert_resource(DebugColliderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .init_resource::<GameWorld>()
            .init_resource::<PlayerLastChunkCoord>()
            .add_event::<PlayerChangedChunkCoordEvent>()
            .add_systems(Update, bind_fresh_game_chunk_entity_to_game_world)
            .add_systems(Update, (
                check_for_player_chunk_position_update,
                update_player_last_chunk_coord,
                spawn_and_destroy_chunks
            ));
            // .add_systems(Update, debug_collider_counts);
            // .add_systems(Update, render_dirty_chunk)
            // .add_systems(Update, enable_close_colliders)
            // .add_systems(Update, disable_far_colliders);
        info!("WorldPlugin loaded");
    }
}