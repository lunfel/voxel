use crate::game_world::coord::{ChunkCoord, GlobalVoxelBlockCoord, LocalVoxelBlockCoord};
use crate::settings::CHUNK_SIZE;
use bevy::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord(pub [f64; 2]);

impl PerlinCoord {
    pub fn from_voxel_block_coord_with_offset(
        block_coord: LocalVoxelBlockCoord,
        chunk_coord: ChunkCoord,
        offset: f64,
    ) -> PerlinCoord {
        PerlinCoord([
            (block_coord.x as f64 + offset) + (chunk_coord.x as f64 * CHUNK_SIZE as f64),
            (block_coord.z as f64 + offset) + (chunk_coord.y as f64 * CHUNK_SIZE as f64),
        ])
    }
}

impl Mul<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn mul(self, rhs: f64) -> Self::Output {
        [self[0] * rhs, self[1] * rhs]
    }
}

impl Div<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn div(self, rhs: f64) -> Self::Output {
        [self[0] / rhs, self[1] / rhs]
    }
}

impl Add<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn add(self, rhs: f64) -> Self::Output {
        [self[0] + rhs, self[1] + rhs]
    }
}

impl Sub<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn sub(self, rhs: f64) -> Self::Output {
        [self[0] - rhs, self[1] - rhs]
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord3d(pub [f64; 3]);

impl PerlinCoord3d {
    pub fn from_voxel_block_coord_with_offset(
        block_coord: LocalVoxelBlockCoord,
        chunk_coord: ChunkCoord,
        offset: f64,
    ) -> PerlinCoord3d {
        PerlinCoord3d([
            (block_coord.x as f64 + offset) + (chunk_coord.x as f64 * CHUNK_SIZE as f64),
            (block_coord.y as f64 + offset) * 5.0,
            (block_coord.z as f64 + offset) + (chunk_coord.y as f64 * CHUNK_SIZE as f64),
        ])
    }
}

impl Mul<f64> for PerlinCoord3d {
    type Output = [f64; 3];

    fn mul(self, rhs: f64) -> Self::Output {
        [self[0] * rhs, self[1] * rhs, self[2] * rhs]
    }
}
