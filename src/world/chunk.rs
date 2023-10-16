use bevy::prelude::*;

use crate::{settings::CHUNK_SIZE, utils::point::Point3D};
use crate::settings::CoordSystemIntegerSize;
use crate::world::block::BlockCoord;

use super::block::GameBlock;

#[derive(Deref, Clone, PartialEq, Eq, Hash, Component)]
pub struct ChunkCoord(Point3D<CoordSystemIntegerSize>);

impl From<Point3D<CoordSystemIntegerSize>> for ChunkCoord {
    fn from(value: Point3D<CoordSystemIntegerSize>) -> Self {
        Self(value)
    }
}

impl From<(CoordSystemIntegerSize, CoordSystemIntegerSize, CoordSystemIntegerSize)> for ChunkCoord {
    fn from(value: (CoordSystemIntegerSize, CoordSystemIntegerSize, CoordSystemIntegerSize)) -> Self {
        Self(Point3D::from(value))
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct ChunkBlocks([[[GameBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

impl ChunkBlocks {
    pub fn blocks_with_coord(&self) -> Vec<(Point3D<i8>, &GameBlock)> {
        let mut pairs: Vec<(Point3D<i8>, &GameBlock)> = Vec::with_capacity(CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    pairs.push(((x as i8, y as i8, z as i8).into(), &self.0[x][y][z]))
                }
            }
        }

        pairs
    }
}

#[derive(Component)]
pub struct GameChunk {
    pub blocks: ChunkBlocks
}

pub type VertexBuffer = Vec<([f32; 3], [f32; 3], [f32; 2])>;

impl GameChunk {
    pub fn new() -> Self {
        Self {
            blocks: default()
        }
    }

    pub fn update_block<P, F>(&mut self, into_coord: &P, update: F)
        where P: Into<BlockCoord> + Clone,
              F: Fn(&mut GameBlock)
    {
        if let Some(block) = self.get_block_mut(into_coord) {
            update(block);
        }
    }

    pub fn get_block<P>(&self, into_coord: &P) -> Option<&GameBlock>
    where P: Into<BlockCoord> + Clone
    {
        let coord: BlockCoord = (*into_coord).clone().into();

        self.blocks.get(coord.x as usize)
            .and_then(|blocks_y| blocks_y.get(coord.y as usize)
                .and_then(|blocks_z| blocks_z.get(coord.z as usize))
            )
    }

    pub fn get_block_mut<P>(&mut self, into_coord: &P) -> Option<&mut GameBlock>
        where P: Into<BlockCoord> + Clone
    {
        let coord: BlockCoord = (*into_coord).clone().into();

        self.blocks.get_mut(coord.x as usize)
            .and_then(|blocks_y| blocks_y.get_mut(coord.y as usize)
                .and_then(|blocks_z| blocks_z.get_mut(coord.z as usize))
            )
    }
}

