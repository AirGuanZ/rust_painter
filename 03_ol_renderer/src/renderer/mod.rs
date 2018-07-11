//! Renderer interface

pub mod path_tracing;
pub mod whitted;

pub mod prelude {
    pub use super::path_tracing::*;
    pub use super::whitted::*;
    use math::{model::*, *};

    pub trait Renderer {
        fn render(&self, r: Ray) -> Color3f;

        fn is_visible(&self, p1: Vec3f, p2: Vec3f) -> bool;
    }
}

pub use self::prelude::*;
