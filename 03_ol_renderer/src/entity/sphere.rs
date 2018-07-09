//! Sphere entity

use entity::*;
use material::*;
use math::*;

pub struct Sphere<M, FM>
where
    M: BxDF,
    FM: Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    sph: model::Sphere,
    fm: Box<FM>,
}

impl Entity for Sphere {
    fn inct(&self, r: Ray) -> Option<Intersection> {
        if let Some((t, p)) = self.sph.nearest_inct(r) {
            let local_y = sph.inct_to_local_y(p);
            let local_x = sph.inct_to_local_x(p);
            let (u, v) = sph.inct_to_uv(p);
            return Intersection {
                t,
                position: p,
                normal: local_y,
                material: fm(p, local_x, local_y, u, v),
            };
        }
        None
    }

    fn to_aabb_bounding(&self) -> AABB {
        self.sph.to_aabb_bounding()
    }
}
