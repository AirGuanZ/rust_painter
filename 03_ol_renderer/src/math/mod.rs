//! Commonly used math tools: matrix, vector, color, ray, ...

extern crate cgmath;
extern crate std;

pub mod color;
pub mod mat;
pub mod model;

pub use self::color::*;
pub use self::mat::*;
pub use self::model::*;

pub use self::cgmath::{Angle, ApproxEq, Deg, Rad};

pub type Radf = Rad<Real>;
pub type Degf = Deg<Real>;

pub type Real = f32;

pub const REAL_MAX: Real = std::f32::MAX;
pub const REAL_PI: Real = 3.1415926535;

pub trait Clamp<T> {
    fn clamp(&self, min_v: T, max_v: T) -> Self;
}

impl Clamp<Real> for Real {
    fn clamp(&self, min_v: Real, max_v: Real) -> Self {
        self.max(min_v).min(max_v)
    }
}
