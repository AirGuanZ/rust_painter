//! Sphere entity

use entity::*;
use material::*;
use math::*;

pub struct Sphere<M, FM>
where
    M: BxDF + 'static,
    FM: Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    sph: model::Sphere,
    fm: Box<FM>,
}

impl<M, FM> Entity for Sphere<M, FM>
where
    M: BxDF + 'static,
    FM: Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    fn inct(&self, r: Ray) -> Option<Intersection> {
        if let Some((t, p)) = self.sph.nearest_inct(r) {
            let local_y = self.sph.inct_to_local_y(p);
            let local_x = self.sph.inct_to_local_x(p);
            let (u, v) = self.sph.inct_to_uv(p);
            let material = (self.fm)(p, local_x, local_y, u, v);
            Some(Intersection {
                t,
                position: p,
                normal: local_y,
                material,
            })
        } else {
            None
        }
    }

    fn to_aabb_bounding(&self) -> AABB {
        self.sph.to_aabb_bounding()
    }
}

impl<M, FM> Sphere<M, FM>
where
    M: BxDF + 'static,
    FM: Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    pub fn new(cen: Vec3f, radius: Real, fm: Box<FM>) -> Self {
        Sphere {
            sph: model::Sphere::new(cen, radius),
            fm,
        }
    }
}
