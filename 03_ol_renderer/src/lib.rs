//! A simple framework for ray-tracing based rendering

pub mod buf;
pub mod camera;
pub mod entity;
pub mod light;
pub mod material;
pub mod math;
pub mod renderer;

pub mod prelude {
    pub use super::buf::*;
    pub use super::camera::*;
    pub use super::entity::*;
    pub use super::light::*;
    pub use super::material::*;
    pub use super::math::*;
    pub use super::renderer::*;
}

pub use self::prelude::*;
