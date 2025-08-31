pub mod coord;
mod generation;
mod player_position;

use crate::game_world::coord::ChunkCoord;
use crate::game_world::generation::{
    begin_generating_map_chunks, receive_generated_map_chunks,
    touch_chunks_around_player_at_interval, ChunkGenerationTaskMap, WorldGenerationState,
};
use crate::game_world::player_position::{
    check_for_player_chunk_position_update, update_player_last_chunk_coord,
};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub use player_position::PlayerChangedChunkCoordEvent;
pub use player_position::PlayerLastChunkCoord;

/// Holds the currently loaded chunks of the game world
/// Depends on ChunkPlugin and PlayerPlugin
pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, tmp_setup)
            .init_resource::<PlayerLastChunkCoord>()
            .init_resource::<WorldGenerationState>()
            .init_resource::<ChunkGenerationTaskMap>()
            .init_resource::<GameWorld>()
            .add_event::<PlayerChangedChunkCoordEvent>()
            .add_systems(
                Update,
                (
                    check_for_player_chunk_position_update,
                    update_player_last_chunk_coord,
                    begin_generating_map_chunks,
                    receive_generated_map_chunks,
                    touch_chunks_around_player_at_interval,
                ),
            );
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for VoxelChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);

#[derive(Component)]
#[require(Transform)]
pub struct WillMakeChunkLoad;
