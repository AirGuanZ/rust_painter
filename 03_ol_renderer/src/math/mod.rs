//! Commonly used math tools: matrix, vector, color, ray, ...

extern crate std;

mod color;
mod mat;
mod ray;

pub use self::color::*;
pub use self::mat::*;
pub use self::ray::*;

pub type Real = f32;

pub const REAL_MAX: Real = std::f32::MAX;

pub trait Clamp<T> {
    fn clamp(&self, min_v: T, max_v: T) -> Self;
}

impl Clamp<Real> for Real {
    fn clamp(&self, min_v: Real, max_v: Real) -> Self {
        self.max(min_v).min(max_v)
    }
}
