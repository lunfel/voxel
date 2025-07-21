pub mod chunk;
pub mod block;
mod procedural;
mod perlin;

use bevy::prelude::*;
use crate::chunk::block::{BlockMaterial, BlockMaterialMap};

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BlockMaterialMap>()
            .init_resource::<BlockMaterial>();
    }
}