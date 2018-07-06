//! Entities in scene

use material::BxDF;
use math::*;

/// Intersection between a ray and an entity. Once the intersection
/// computed, the entity becomes totally useless.
pub struct Intersection<'a> {
    pub position: Pnt3f,
    pub normal: Vec3f,
    pub material: &'a Box<BxDF>,
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
    fn to_aabb_bounding(&self) -> AABB;
}
