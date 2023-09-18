#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Point3D<T> {
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
        }
    };
}

point_neighbour_impl!(u8);
point_neighbour_impl!(usize);
point_neighbour_impl!(i8);

impl From<Point3D<usize>> for Point3D<i8> {
    fn from(value: Point3D<usize>) -> Self {
        Self {
            x: value.x as i8,
            y: value.y as i8,
            z: value.z as i8
        }
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

impl<T> From<(T, T, T)> for Point3D<T> {
    fn from(value: (T, T, T)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}

