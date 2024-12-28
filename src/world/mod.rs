use bevy::app::App;
use bevy::prelude::*;

use crate::world::generate_as_we_go::{begin_generating_map_chunks, check_for_player_chunk_position_update, receive_generated_map_chunks, update_player_last_chunk_coord, ChunkGenerationTaskMap, PlayerChangedChunkCoordEvent, PlayerLastChunkCoord};
// use systems::chunk::DebugColliderTimer;
use game_world::GameWorld;

pub mod block;
pub mod chunk;
pub mod game_world;
pub mod world_generation;
mod generate_as_we_go;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        info!("WorldPlugin initializing");
        app
            // .insert_resource(DebugColliderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .init_resource::<GameWorld>()
            .init_resource::<PlayerLastChunkCoord>()
            .init_resource::<ChunkGenerationTaskMap>()
            .add_event::<PlayerChangedChunkCoordEvent>()
            .add_systems(Update, (
                check_for_player_chunk_position_update,
                update_player_last_chunk_coord,
                begin_generating_map_chunks,
                receive_generated_map_chunks
            ));
            // .add_systems(Update, debug_collider_counts);
            // .add_systems(Update, render_dirty_chunk)
            // .add_systems(Update, enable_close_colliders)
            // .add_systems(Update, disable_far_colliders);
        info!("WorldPlugin loaded");
    }
}