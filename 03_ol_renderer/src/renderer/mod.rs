//! Renderer interface

pub mod whitted;

pub use self::whitted::*;
use math::*;

pub trait Renderer {
    fn Render(&self, r: Ray) -> Color3f;
}
