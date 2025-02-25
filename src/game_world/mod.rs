pub mod coord;
mod temporary;

use crate::game_world::coord::ChunkCoord;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game_world::temporary::tmp_setup;

/// Holds the currently loaded chunks of the game world
pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tmp_setup);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for VoxelChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);

#[derive(Component)]
#[require(Transform)]
pub struct WillMakeChunkLoad;