use bevy::prelude::Deref;
use crate::settings::CoordSystemIntegerSize;
use crate::utils::point::Point3D;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameBlockType {
    #[default]
    Empty,
    Rock,
    Ground
}

#[derive(Default, Copy, Clone)]
pub struct GameBlock {
    pub block_type: GameBlockType,
    pub is_fully_surrounded: bool
}

/// Block coordinate inside a chunk
#[derive(Deref, Clone, PartialEq, Eq, Hash)]
pub struct BlockCoord(Point3D<CoordSystemIntegerSize>);

impl From<Point3D<CoordSystemIntegerSize>> for BlockCoord {
    fn from(value: Point3D<CoordSystemIntegerSize>) -> Self {
        Self(value)
    }
}

impl From<(CoordSystemIntegerSize, CoordSystemIntegerSize, CoordSystemIntegerSize)> for BlockCoord {
    fn from(value: (CoordSystemIntegerSize, CoordSystemIntegerSize, CoordSystemIntegerSize)) -> Self {
        Self(Point3D::from(value))
    }
}

impl From<(usize, usize, usize)> for BlockCoord {
    fn from(value: (usize, usize, usize)) -> Self {
        BlockCoord(Point3D {
            x: value.0 as CoordSystemIntegerSize,
            y: value.1 as CoordSystemIntegerSize,
            z: value.2 as CoordSystemIntegerSize
        })
    }
}

// impl TryFrom<Point3D<i8>> for BlockCoord {
//     type Error = TryFromIntError;
//     fn try_from(value: Point3D<i8>) -> Result<Self, Self::Error> {
//         value.try_into().map(|p| BlockCoord(p))
//     }
// }