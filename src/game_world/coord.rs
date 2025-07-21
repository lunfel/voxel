use std::ops::Add;
use bevy::prelude::{Component, Deref, DerefMut, Transform};
use bevy_rapier3d::na::{Point2, Point3};
use crate::settings::{CoordSystemIntegerSize, CHUNK_HEIGHT, CHUNK_SIZE, MAX_OFFSET};

/// ChunkCoord is the coordinate of the chunk in using the
/// value 1 for each chunk. Multiply ChunkCoord by CHUNK_SIZE
/// to get offset in real world
#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct ChunkCoord(pub Point2<CoordSystemIntegerSize>);

impl From<ChunkCoord> for Transform {
    fn from(chunk: ChunkCoord) -> Self {
        Transform::from_xyz(
            (chunk.x * CHUNK_SIZE) as f32,
            0.0,
            (chunk.y * CHUNK_SIZE) as f32
        )
    }
}

#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct GlobalVoxelBlockCoord(Point3<CoordSystemIntegerSize>);

#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct LocalVoxelBlockCoord(pub Point3<CoordSystemIntegerSize>);

impl LocalVoxelBlockCoord {
    pub fn is_valid_chunk_voxel_coord(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.z >= 0 && self.x < CHUNK_SIZE && self.y < CHUNK_HEIGHT && self.z < CHUNK_SIZE
    }

    pub fn front_neighbor(&self) -> Option<Self> {
        self.z.checked_add(1)
            .map(|z| Self (Point3::new(self.x, self.y, z)))
    }

    pub fn back_neighbor(&self) -> Option<Self> {
        self.z.checked_sub(1)
            .map(|z| Self (Point3::new(self.x, self.y, z)))
    }

    pub fn right_neighbor(&self) -> Option<Self> {
        self.x.checked_add(1)
            .map(|x| Self (Point3::new(x, self.y, self.z)))
    }

    pub fn left_neighbor(&self) -> Option<Self> {
        self.x.checked_sub(1)
            .map(|x| Self (Point3::new(x, self.y, self.z)))
    }

    pub fn top_neighbor(&self) -> Option<Self> {
        self.y.checked_add(1)
            .map(|y| Self (Point3::new(self.x, y, self.z)))
    }

    pub fn bottom_neighbor(&self) -> Option<Self> {
        self.y.checked_sub(1)
            .map(|y| Self (Point3::new(self.x, y, self.z)))
    }

    pub fn neighbors(&self) -> [Option<Self>; 6] {
        [
            self.front_neighbor(),
            self.back_neighbor(),
            self.right_neighbor(),
            self.left_neighbor(),
            self.top_neighbor(),
            self.bottom_neighbor()
        ]
    }
}

#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct LocalVoxelBlockOffset(pub usize);

impl Add<[CoordSystemIntegerSize; 3]> for LocalVoxelBlockOffset {
    type Output = Option<LocalVoxelBlockOffset>;

    fn add(self, rhs: [CoordSystemIntegerSize; 3]) -> Self::Output {
        let value = self.0;
        let rhs_offset = rhs[0] + (rhs[2] * CHUNK_SIZE) + (rhs[1] * CHUNK_SIZE * CHUNK_SIZE);
        let new_offset = value as CoordSystemIntegerSize + rhs_offset;

        match new_offset {
            0..MAX_OFFSET => Some(LocalVoxelBlockOffset(new_offset as usize)),
            _ => None,
        }
    }
}

impl Add<[CoordSystemIntegerSize; 3]> for &LocalVoxelBlockOffset {
    type Output = Option<LocalVoxelBlockOffset>;

    fn add(self, rhs: [CoordSystemIntegerSize; 3]) -> Self::Output {
        let value = self.0;
        let rhs_offset = rhs[0] + (rhs[2] * CHUNK_SIZE) + (rhs[1] * CHUNK_SIZE * CHUNK_SIZE);
        let new_offset = value as CoordSystemIntegerSize + rhs_offset;

        match new_offset {
            0..MAX_OFFSET => Some(LocalVoxelBlockOffset(new_offset as usize)),
            _ => None,
        }
    }
}

impl Add<[CoordSystemIntegerSize; 3]> for LocalVoxelBlockCoord {
    type Output = Option<LocalVoxelBlockCoord>;

    fn add(self, rhs: [CoordSystemIntegerSize; 3]) -> Self::Output {
        let new_x = self.x + rhs[0];
        let new_y = self.y + rhs[1];
        let new_z = self.z + rhs[2];

        match (new_x, new_y, new_z) {
            (0..CHUNK_SIZE, 0..CHUNK_HEIGHT, 0..CHUNK_SIZE) => {
                Some(LocalVoxelBlockCoord(Point3::new(new_x, new_y, new_z)))
            },
            _ => None,
        }
    }
}

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

impl From<[CoordSystemIntegerSize; 3]> for LocalVoxelBlockCoord {
    fn from(value: [CoordSystemIntegerSize; 3]) -> Self {
        Self(Point3::new(
            value[0] as CoordSystemIntegerSize,
            value[1] as CoordSystemIntegerSize,
            value[2] as CoordSystemIntegerSize,
        ))
    }
}

impl TryFrom<[CoordSystemIntegerSize; 3]> for LocalVoxelBlockOffset {
    type Error = ();
    fn try_from(value: [CoordSystemIntegerSize; 3]) -> Result<Self, Self::Error> {
        match (value[0], value[1], value[2]) {
            (0..CHUNK_SIZE, 0..CHUNK_HEIGHT, 0..CHUNK_SIZE) => Ok(Self((value[0] + (value[2] * CHUNK_SIZE) + (value[1] * CHUNK_SIZE * CHUNK_SIZE)) as usize)),
            _ => Err(()),
        }
    }
}

/// This implementation assumes that the LocalVoxelBlockCoord is inside the bounds
/// of the chunk. Otherwise, the behavior is unpredictable.
impl From<LocalVoxelBlockCoord> for LocalVoxelBlockOffset {
    fn from(value: LocalVoxelBlockCoord) -> Self {
        Self ((value.x + (value.z * CHUNK_SIZE) + (value.y * CHUNK_SIZE * CHUNK_SIZE)) as usize)
    }
}

impl From<&LocalVoxelBlockCoord> for LocalVoxelBlockOffset {
    fn from(local_voxel_block_coord: &LocalVoxelBlockCoord) -> Self {
        LocalVoxelBlockOffset::from(*local_voxel_block_coord)
    }
}

impl From<LocalVoxelBlockOffset> for LocalVoxelBlockCoord {
    fn from(value: LocalVoxelBlockOffset) -> Self {
        let chunk_size = CHUNK_SIZE as usize;

        let y = value.0 / (chunk_size * chunk_size);
        let z = (value.0 % (chunk_size * chunk_size)) / chunk_size;
        let x = value.0 % chunk_size;

        Self (Point3::new(
            x as CoordSystemIntegerSize,
            y as CoordSystemIntegerSize,
            z as CoordSystemIntegerSize
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::char::MAX;
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

    #[test]
    fn local_coord_to_offset() {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(x, y, z)));

                    assert!(offset.is_ok());
                }
            }
        }
    }

    #[test]
    fn offset_to_local_coord() {
        let cases = vec![
            (0, (0, 0, 0)),
            (15, (15, 0, 0)),
            (16, (0, 0, 1)),
            (31, (15, 0, 1)),
            (257, (1, 1, 0)),
        ];

        for case in cases {
            let offset = LocalVoxelBlockOffset(case.0);

            let coord = LocalVoxelBlockCoord::from(offset);

            assert_eq!(coord.x, case.1.0);
            assert_eq!(coord.y, case.1.1);
            assert_eq!(coord.z, case.1.2);
        }
    }

    #[test]
    fn local_coord_outside_bounds() {
        let zero = LocalVoxelBlockCoord(Point3::new(0, 0, 0));
        let limit = LocalVoxelBlockCoord(Point3::new(CHUNK_SIZE - 1, CHUNK_HEIGHT - 1, CHUNK_SIZE - 1));

        let cases = vec![
            zero + [-1, 0, 0],
            zero + [0, -1, 0],
            zero + [0, 0, -1],
            zero + [-1, -1, 0],
            zero + [0, -1, -1],
            zero + [-1, 0, -1],
            zero + [-1, -1, -1],

            limit + [1, 0, 0],
            limit + [0, 1, 0],
            limit + [0, 0, 1],
            limit + [1, 1, 0],
            limit + [0, 1, 1],
            limit + [1, 0, 1],
            limit + [1, 1, 1],
        ];

        for case in cases {
            assert!(case.is_none());
        }
    }

    #[test]
    fn block_offset_add_tuple3() {
        let offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(0, 0, 0))).unwrap();
        let offset = offset + [0, 0, 1];

        let expected_offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(0, 0, 1))).unwrap();

        match offset {
            Some(v) => assert_eq!(expected_offset, v),
            None => assert!(false),
        }

        let offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(4, 9, 14))).unwrap();
        let offset = offset + [3, 2, 0];

        let expected_offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(7, 11, 14))).unwrap();

        match offset {
            Some(v) => assert_eq!(expected_offset, v),
            None => assert!(false),
        }

        let offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(CHUNK_SIZE - 1, CHUNK_HEIGHT - 1, CHUNK_SIZE - 1))).unwrap();
        let offset = offset + [1, 1, 1];
        
        println!("{:?}", offset);

        assert!(offset.is_none());
    }

    #[test]
    fn block_offset_add_tuple3_specific_case() {
        let offset = LocalVoxelBlockOffset(1);
        let offset = offset + [-1, 0, 0];

        let expected_offset = LocalVoxelBlockOffset::try_from(LocalVoxelBlockCoord(Point3::new(0, 0, 0))).unwrap();

        match offset {
            Some(v) => assert_eq!(expected_offset, v),
            None => assert!(false),
        }
    }
}