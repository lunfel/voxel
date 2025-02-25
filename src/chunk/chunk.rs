use crate::chunk::block::VoxelBlock;
use crate::game_world::coord::LocalVoxelBlockOffset;
use crate::settings::{CHUNK_HEIGHT, CHUNK_SIZE};
use bevy::prelude::*;

#[derive(Deref, DerefMut, Debug, Clone)]
pub struct ChunkBlocks([VoxelBlock; (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize]);

impl Default for ChunkBlocks {
    fn default() -> Self {
        ChunkBlocks([VoxelBlock::default(); (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize])
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

    pub fn update_block<P, F>(&mut self, into_coord: &P, update: F)
    where P: Into<LocalVoxelBlockOffset> + Clone,
          F: Fn(&mut VoxelBlock)
    {
        if let Some(block) = self.get_block_mut(into_coord) {
            update(block);
        }
    }

    pub fn get_block<P>(&self, into_coord: &P) -> Option<&VoxelBlock>
    where P: Into<LocalVoxelBlockOffset> + Clone
    {
        let coord = into_coord.clone().into();
        
        self.blocks.get(*coord)
    }

    pub fn get_block_mut<P>(&mut self, into_coord: &P) -> Option<&mut VoxelBlock>
    where P: Into<LocalVoxelBlockOffset> + Clone
    {
        let coord = into_coord.clone().into();

        self.blocks.get_mut(*coord)
    }
}