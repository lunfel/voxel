use bevy::{prelude::Resource, utils::HashMap};

use crate::Point3D;

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: HashMap<Point3D, GameChunk>
}

pub struct GameChunk {
    pub blocks: [[[GameBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum GameBlockType {
    #[default]
    Empty,
    Ground
}

#[derive(Default, Copy, Clone)]
pub struct GameBlock {
    pub block_type: GameBlockType
}

impl GameChunk {
    pub fn new() -> Self {
        GameChunk {
            blocks: [[[GameBlock::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] 
        }
    }

    pub fn get_block(&self, coord: &Point3D) -> Option<&GameBlock> {
        self.blocks.get(coord.0 as usize)
            .and_then(|blocks_y| blocks_y.get(coord.1 as usize)
                .and_then(|blocks_z| blocks_z.get(coord.2 as usize))
            )
        
    }
}

