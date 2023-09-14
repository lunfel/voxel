use std::ops::Deref;

use crate::{settings::CHUNK_SIZE, utils::point::Point3D};

use super::block::GameBlock;

#[derive(Clone, PartialEq, Eq)]
pub struct ChunkCoord(Point3D<usize>);

impl Deref for ChunkCoord {
    type Target = Point3D<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct GameChunk {
    pub blocks: [[[GameBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
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

