//! Camera model

pub mod perspective;

pub mod prelude {
    pub use super::perspective::*;
    use math::*;

    pub trait Camera {
        fn scr_to_ray(&self, scr_point: Vec2f) -> Ray;
    }
}

pub use self::prelude::*;
