use bevy::prelude::{Component, Deref, DerefMut};
use bevy_rapier3d::na::{Point2, Point3};
use crate::settings::{CoordSystemIntegerSize, CHUNK_SIZE};

/// ChunkCoord is the coordinate of the chunk in using the
/// value 1 for each chunk. Multiply ChunkCoord by CHUNK_SIZE
/// to get offset in real world
#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct ChunkCoord(Point2<CoordSystemIntegerSize>);

#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct GlobalVoxelBlockCoord(Point3<CoordSystemIntegerSize>);

#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct LocalVoxelBlockCoord(Point3<CoordSystemIntegerSize>);

impl From<(ChunkCoord, LocalVoxelBlockCoord)> for GlobalVoxelBlockCoord {
    fn from((chunk_coord, local_block_coord): (ChunkCoord, LocalVoxelBlockCoord)) -> Self {
        GlobalVoxelBlockCoord(Point3::new(
            chunk_coord.x * CHUNK_SIZE + local_block_coord.x,
            local_block_coord.y,
            chunk_coord.y * CHUNK_SIZE + local_block_coord.z,
        ))
    }
}

impl From<GlobalVoxelBlockCoord> for (ChunkCoord, LocalVoxelBlockCoord) {
    fn from(global_voxel_block_coord: GlobalVoxelBlockCoord) -> Self {
        (ChunkCoord(Point2::new(
            (global_voxel_block_coord.x as f32 / CHUNK_SIZE as f32).floor() as CoordSystemIntegerSize,
            (global_voxel_block_coord.z as f32 / CHUNK_SIZE as f32).floor() as CoordSystemIntegerSize,
        )), LocalVoxelBlockCoord(Point3::new(
            ((global_voxel_block_coord.x % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
            global_voxel_block_coord.y,
            ((global_voxel_block_coord.z % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        )))
    }
}

impl From<&GlobalVoxelBlockCoord> for (ChunkCoord, LocalVoxelBlockCoord) {
    fn from(global_voxel_block_coord: &GlobalVoxelBlockCoord) -> Self {
        (*global_voxel_block_coord).into()
    }
}

impl From<&mut GlobalVoxelBlockCoord> for (ChunkCoord, LocalVoxelBlockCoord) {
    fn from(global_voxel_block_coord: &mut GlobalVoxelBlockCoord) -> Self {
        (*global_voxel_block_coord).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_global_to_local() {
        // Based on a CHUNK_SIZE of 16
        let global = GlobalVoxelBlockCoord(Point3::new(0, 0, 0));

        let (chunk_coord, local_block_coord) = global.into();

        assert_eq!(chunk_coord.x, 0);
        assert_eq!(chunk_coord.y, 0);

        assert_eq!(local_block_coord.x, 0);
        assert_eq!(local_block_coord.y, 0);
        assert_eq!(local_block_coord.z, 0);

        let global = GlobalVoxelBlockCoord(Point3::new(16, 0, 32));

        let (chunk_coord, local_block_coord) = global.into();

        println!("{:?} - {:?}", chunk_coord, local_block_coord);

        assert_eq!(chunk_coord.x, 1);
        assert_eq!(chunk_coord.y, 2);

        assert_eq!(local_block_coord.x, 0);
        assert_eq!(local_block_coord.y, 0);
        assert_eq!(local_block_coord.z, 0);

        let global = GlobalVoxelBlockCoord(Point3::new(-1, 100, -17));

        let (chunk_coord, local_block_coord) = global.into();

        println!("{:?} - {:?}", chunk_coord, local_block_coord);

        assert_eq!(chunk_coord.x, -1);
        assert_eq!(chunk_coord.y, -2);

        assert_eq!(local_block_coord.x, 15);
        assert_eq!(local_block_coord.y, 100);
        assert_eq!(local_block_coord.z, 15);

        let global = GlobalVoxelBlockCoord(Point3::new(100, 22, -100));

        let (chunk_coord, local_block_coord) = global.into();

        println!("{:?} - {:?}", chunk_coord, local_block_coord);

        assert_eq!(chunk_coord.x, 6);
        assert_eq!(chunk_coord.y, -7);

        assert_eq!(local_block_coord.x, 4);
        assert_eq!(local_block_coord.y, 22);
        assert_eq!(local_block_coord.z, 12);
    }

    #[test]
    fn from_local_to_global() {
        let chunk_coord = ChunkCoord(Point2::new(0, 0));
        let local_block_coord = LocalVoxelBlockCoord(Point3::new(0, 0, 0));

        let global_coord: GlobalVoxelBlockCoord = (chunk_coord, local_block_coord).into();

        assert_eq!(global_coord.x, 0);
        assert_eq!(global_coord.y, 0);
        assert_eq!(global_coord.z, 0);

        let chunk_coord = ChunkCoord(Point2::new(3, 4));
        let local_block_coord = LocalVoxelBlockCoord(Point3::new(2, 5, 8));

        let global_coord: GlobalVoxelBlockCoord = (chunk_coord, local_block_coord).into();

        assert_eq!(global_coord.x, 50);
        assert_eq!(global_coord.y, 5);
        assert_eq!(global_coord.z, 72);

        let chunk_coord = ChunkCoord(Point2::new(-2, -4));
        let local_block_coord = LocalVoxelBlockCoord(Point3::new(3, 7, 15));

        let global_coord: GlobalVoxelBlockCoord = (chunk_coord, local_block_coord).into();

        assert_eq!(global_coord.x, -29);
        assert_eq!(global_coord.y, 7);
        assert_eq!(global_coord.z, -49);
    }
}