//! Camera model

pub mod perspective;

use math::*;
pub use self::perspective::*;

pub trait Camera {
    fn scr_to_ray(&self, scr_point: Vec2f) -> Ray;
}
