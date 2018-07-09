//! Entities in scene

pub mod sphere;

pub mod prelude {
    pub use super::sphere::*;
}

pub use self::prelude::*;
use material::*;
use math::*;

/// Intersection between a ray and an entity. Once the intersection
/// computed, the entity becomes totally useless.
pub struct Intersection {
    pub t: Real,
    pub position: Vec3f,
    pub normal: Vec3f,
    pub material: Box<BxDF>,
}

impl Intersection {
    pub fn nearer(self, other: Intersection) -> Intersection {
        assert!(self.t >= 0.0 && other.t >= 0.0);
        if self.t < other.t {
            self
        } else {
            other
        }
    }
}

/// (Renderable) Entities in scene.
///
/// # Entity
///
/// An entity doesn't need to be a single 'object'.
/// An entity can also be an BVH tree of other entities,
/// an visivle light source, or anything else that can be in the rendered scene.
pub trait Entity {
    fn inct(&self, r: Ray) -> Option<Intersection>;
    fn to_aabb_bounding(&self) -> model::AABB;
}
