#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl Point3D<i8> {
    pub fn front_neighbor(&self) -> Self {
        Self {
            z: self.z + 1,
            ..*self
        }
    }

    pub fn back_neighbor(&self) -> Self {
        Self {
            z: self.z - 1,
            ..*self
        }
    }

    pub fn left_neighbor(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }

    pub fn right_neighbor(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }

    pub fn top_neighbor(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }

    pub fn bottom_neighbor(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }
}

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

