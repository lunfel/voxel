use crate::utils::point::Point3D;

pub struct Cube<T: Copy + Ord> {
    pub min: Point3D<T>,
    pub max: Point3D<T>
}

impl<T: Copy + Ord> Cube<T> {
    pub fn from_points(min: Point3D<T>, max: Point3D<T>) -> Self {
        let (min, max) = min.min_max(&max);

        Cube { min, max }
    }
}
