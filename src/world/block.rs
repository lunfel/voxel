use std::num::TryFromIntError;
use bevy::prelude::Deref;
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
    pub block_type: GameBlockType
}

/// Block coordinate inside a chunk
#[derive(Deref, Clone, PartialEq, Eq, Hash)]
pub struct BlockCoord(Point3D<usize>);

impl From<Point3D<usize>> for BlockCoord {
    fn from(value: Point3D<usize>) -> Self {
        Self(value)
    }
}

impl From<(usize, usize, usize)> for BlockCoord {
    fn from(value: (usize, usize, usize)) -> Self {
        Self(Point3D::from(value))
    }
}

impl TryFrom<Point3D<i8>> for BlockCoord {
    type Error = TryFromIntError;
    fn try_from(value: Point3D<i8>) -> Result<Self, Self::Error> {
        value.try_into().map(|p| BlockCoord(p))
    }
}