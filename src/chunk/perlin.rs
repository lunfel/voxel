use bevy::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord(pub [f64; 2]);

impl Mul<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn mul(self, rhs: f64) -> Self::Output {
        [
            self[0] * rhs,
            self[1] * rhs
        ]
    }
}

impl Div<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn div(self, rhs: f64) -> Self::Output {
        [
            self[0] / rhs,
            self[1] / rhs
        ]
    }
}

impl Add<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn add(self, rhs: f64) -> Self::Output {
        [
            self[0] + rhs,
            self[1] + rhs
        ]
    }
}

impl Sub<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn sub(self, rhs: f64) -> Self::Output {
        [
            self[0] - rhs,
            self[1] - rhs
        ]
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord3d(pub [f64; 3]);

impl Mul<f64> for PerlinCoord3d {
    type Output = [f64; 3];

    fn mul(self, rhs: f64) -> Self::Output {
        [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
        ]
    }
}