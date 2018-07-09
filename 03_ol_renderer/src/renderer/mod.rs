//! Renderer interface

pub mod whitted;

pub mod prelude {
    pub use super::whitted::*;
}

pub use self::prelude::*;
use math::{model::*, *};

pub trait Renderer {
    fn render(&self, r: Ray) -> Color3f;
}
