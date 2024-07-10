use std::cmp::{max, min};
use crate::settings::CoordSystemIntegerSize;
use crate::utils::cube::Cube;

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Point3D<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

macro_rules! point_neighbour_impl {
    ($t:ty) => {
        impl Point3D<$t> {
            pub fn front_neighbor(&self) -> Option<Self> {
                self.z.checked_add(1)
                    .map(|z| Self {
                            z,
                            ..*self
                        }
                    )
            }

            pub fn back_neighbor(&self) -> Option<Self> {
                self.z.checked_sub(1)
                    .map(|z| Self {
                            z,
                            ..*self
                        }
                    )
            }

            pub fn right_neighbor(&self) -> Option<Self> {
                self.x.checked_add(1)
                    .map(|x| Self {
                            x,
                            ..*self
                        }
                    )
            }

            pub fn left_neighbor(&self) -> Option<Self> {
                self.x.checked_sub(1)
                    .map(|x| Self {
                            x,
                            ..*self
                        }
                    )
            }

            pub fn top_neighbor(&self) -> Option<Self> {
                self.y.checked_add(1)
                    .map(|y| Self {
                            y,
                            ..*self
                        }
                    )
            }

            pub fn bottom_neighbor(&self) -> Option<Self> {
                self.y.checked_sub(1)
                    .map(|y| Self {
                            y,
                            ..*self
                        }
                    )
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
    };
}

point_neighbour_impl!(u8);
point_neighbour_impl!(usize);
point_neighbour_impl!(i8);
point_neighbour_impl!(i32);
impl From<Point3D<usize>> for Point3D<i8> {
    fn from(value: Point3D<usize>) -> Self {
        Self {
            x: value.x as i8,
            y: value.y as i8,
            z: value.z as i8
        }
    }
}

impl<T: Copy + Ord> Point3D<T> {
    pub fn is_inside_of_cube(&self, cube: &Cube<T>) -> bool {
        cube.min.x <= self.x && self.x <= cube.max.x
            && cube.min.y <= self.y && self.y <= cube.max.y
            && cube.min.z <= self.z && self.z <= cube.max.z
    }

    pub fn min(&self, other: &Point3D<T>) -> Self {
        Self {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
            z: min(self.z, other.z),
        }
    }

    pub fn max(&self, other: &Point3D<T>) -> Self {
        Self {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            z: max(self.z, other.z),
        }
    }

    pub fn min_max(&self, other: &Point3D<T>) -> (Self, Self) {
        (
            self.min(other),
            self.max(other)
        )
    }
}

impl TryFrom<Point3D<i8>> for Point3D<usize> {
    type Error = std::num::TryFromIntError;
    fn try_from(value: Point3D<i8>) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;
        let z = value.z.try_into()?;

        Ok(Self {
            x,
            y,
            z
        })
    }
}

impl<T: Copy> From<(T, T, T)> for Point3D<T> {
    fn from(value: (T, T, T)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
