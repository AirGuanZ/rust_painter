//! Geometry models for entities
//!
//! # Why no 'Model' trait
//!
//! Any geometry model will be hidden in rendering interface,
//! thus a 'Model' trait is unnecessary. Actually, I don't know how
//! to properly define it. On the one hand, if we have a parameterized geometry model,
//! ray-model intersections will carry different members for different kinds of models.
//! So we will need a 'Intersection' trait, which may make it complicated to implement
//! entity, renderer, and anything relating to intersections. On the otherhand, if we have a
//! universal interface for all kinds of geometry models with only one kind of intersection,
//! this interface seems to be useless because all models are invisible to other modules except
//! entities.
//!

pub mod aabb;
pub mod ray;
pub mod sphere;

pub mod prelude {
    pub use super::aabb::*;
    pub use super::ray::*;
    pub use super::sphere::*;
}

pub use self::prelude::*;
