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

mod aabb;
mod sphere;

pub use model::aabb::AABB;
pub use model::sphere::Sphere;
