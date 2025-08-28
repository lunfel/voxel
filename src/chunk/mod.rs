pub mod chunk;
pub mod block;
pub mod procedural;
mod perlin;
pub mod noise;

use bevy::prelude::*;
use crate::chunk::block::{BlockMaterial, BlockMaterialMap};
use crate::chunk::chunk::add_new_chunks_to_game_world;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BlockMaterialMap>()
            .init_resource::<BlockMaterial>()
            .add_systems(Update, add_new_chunks_to_game_world);
    }
}