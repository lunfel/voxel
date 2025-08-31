pub mod block;
pub mod voxel_chunk;
pub mod noise;
pub mod procedural;

use crate::chunk::block::{BlockMaterial, BlockMaterialMap};
use crate::chunk::voxel_chunk::add_new_chunks_to_game_world;
use bevy::prelude::*;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockMaterialMap>()
            .init_resource::<BlockMaterial>()
            .add_systems(Update, add_new_chunks_to_game_world);
    }
}
