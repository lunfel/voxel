use bevy::{prelude::Resource, utils::HashMap};

use crate::utils::point::Point3D;

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: HashMap<Point3D<usize>, GameChunk>
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

    pub fn get_block<P>(&self, maybe_into_coord: &P) -> Option<&GameBlock>
    where P: TryInto<Point3D<usize>> + Clone
    {
        let res: Result<Point3D<usize>, _> = (*maybe_into_coord).clone().try_into();

        if let Ok(coord) = res {
            self.blocks.get(coord.x)
                .and_then(|blocks_y| blocks_y.get(coord.y)
                    .and_then(|blocks_z| blocks_z.get(coord.z))
                )
        } else {
            None
        }
    }
}

