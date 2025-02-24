use bevy::prelude::*;
use crate::chunk::block::VoxelBlock;
use crate::settings::{CHUNK_HEIGHT, CHUNK_SIZE};

#[derive(Deref, DerefMut, Debug, Clone)]
pub struct ChunkBlocks([[[VoxelBlock; CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]);

impl Default for ChunkBlocks {
    fn default() -> Self {
        ChunkBlocks([[[VoxelBlock::default(); CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize])
    }
}

pub struct VoxelChunk {
    pub blocks: ChunkBlocks
}

impl VoxelChunk {
    pub fn new() -> Self {
        Self {
            blocks: ChunkBlocks::default()
        }
    }
}