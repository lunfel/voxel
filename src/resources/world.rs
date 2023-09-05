use bevy::{prelude::Resource, utils::HashMap};

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: HashMap<(u8, u8, u8), GameChunk>
}

pub struct GameChunk {
    pub blocks: [[[GameBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
}

#[derive(Default, Copy, Clone)]
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
}

