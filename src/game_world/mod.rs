pub mod coord;
mod temporary;
mod generation;
mod player_position;

use crate::game_world::coord::ChunkCoord;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game_world::generation::WorldGenerationState;
use crate::game_world::player_position::{check_for_player_chunk_position_update, update_player_last_chunk_coord, PlayerChangedChunkCoordEvent, PlayerLastChunkCoord};
use crate::game_world::temporary::tmp_setup;
use crate::logging::LogIntervalTimer;

/// Holds the currently loaded chunks of the game world
/// Depends on ChunkPlugin and PlayerPlugin
pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tmp_setup)
            .init_resource::<PlayerLastChunkCoord>()
            .init_resource::<WorldGenerationState>()
            .add_event::<PlayerChangedChunkCoordEvent>()
            .add_systems(Update, (
                check_for_player_chunk_position_update,
                update_player_last_chunk_coord
            ));
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for VoxelChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);

#[derive(Component)]
#[require(Transform)]
pub struct WillMakeChunkLoad;