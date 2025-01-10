use std::time::Duration;
use bevy::app::App;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::world::generate_as_we_go::{begin_generating_map_chunks, check_for_player_chunk_position_update, count_number_of_triangles_in_chunk_meshes, receive_generated_map_chunks, remove_chunks_that_are_stale, touch_chunks_around_player_at_interval, update_player_last_chunk_coord, ChunkGenerationTaskMap, PlayerChangedChunkCoordEvent, PlayerLastChunkCoord};
// use systems::chunk::DebugColliderTimer;
use game_world::GameWorld;
use crate::world::game_world::add_pending_chunks_to_game_world;

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
            .add_systems(Update, add_pending_chunks_to_game_world)
            .add_systems(Update, (
                check_for_player_chunk_position_update,
                update_player_last_chunk_coord,
                begin_generating_map_chunks,
                receive_generated_map_chunks,
                touch_chunks_around_player_at_interval.run_if(on_timer(Duration::from_secs(1))),
                remove_chunks_that_are_stale.run_if(on_timer(Duration::from_secs(1))),
                count_number_of_triangles_in_chunk_meshes.run_if(on_timer(Duration::from_secs(1)))
            ));
            // .add_systems(Update, debug_collider_counts);
            // .add_systems(Update, render_dirty_chunk)
            // .add_systems(Update, enable_close_colliders)
            // .add_systems(Update, disable_far_colliders);
        info!("WorldPlugin loaded");
    }
}