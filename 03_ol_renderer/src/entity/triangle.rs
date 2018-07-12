//! Triangle entity

use entity::*;
use material::*;
use math::*;

pub struct Triangle<M, FM>
where
    M: BxDF + 'static,
    FM: Sync + Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    tri: model::Triangle,
    fm: Box<FM>
}

impl<M, FM> Entity for Triangle<M, FM>
where
    M: BxDF + 'static,
    FM: Sync + Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    fn inct(&self, r: Ray) -> Option<Intersection> {
        if let Some(inct) = self.tri.nearest_inct(r.clone()) {
            let p = r.t_to_point(inct.t);
            let n = self.tri.normal(&r);
            Some(Intersection {
                t: inct.t,
                position: p,
                normal: n,
                material: (self.fm)(p, ZERO_VEC3, n, inct.beta, inct.gamma),
            })
        } else {
            None
        }
    }

    fn has_inct(&self, r: Ray) -> Option<(Real, Vec3f)> {
        match self.tri.is_intersected(r.clone()) {
            Some(t) => Some((t, r.t_to_point(t))),
            None => None
        }
    }
}

impl<M, FM> Triangle<M, FM>
where
    M: BxDF + 'static,
    FM: Sync + Fn(Vec3f, Vec3f, Vec3f, Real, Real) -> Box<M>,
{
    pub fn new(vtx: [Vec3f; 3], fm: Box<FM>) -> Self {
        Triangle {
            tri: model::Triangle::new(vtx),
            fm,
        }
    }
}
