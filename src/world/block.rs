use bevy::prelude::Deref;
use crate::settings::CoordSystemIntegerSize;
use crate::utils::point::Point3D;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameBlockType {
    #[default]
    Empty,
    Rock,
    Ground,
    Gem
}

#[derive(Default, Copy, Clone, Debug)]
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