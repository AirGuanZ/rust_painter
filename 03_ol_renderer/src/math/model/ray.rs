//! Implementation of p = p0 + t * d

use math::{mat::*, Clamp, Real};

#[derive(Clone)]
pub struct Ray {
    pub p: Pnt3f,
    pub d: Vec3f,
}

#[derive(Clone)]
pub struct RayT {
    pub r: Ray,
    pub t: Real,
}

impl Ray {
    pub fn new(p: Pnt3f, d: Vec3f) -> Ray {
        Ray {
            p,
            d: d.normalize(),
        }
    }

    pub fn t_to_point(&self, t: Real) -> Pnt3f {
        self.p + t * self.d
    }
}

impl Clamp<Real> for RayT {
    fn clamp(&self, min_v: Real, max_v: Real) -> RayT {
        RayT {
            t: self.t.clamp(min_v, max_v),
            ..self.clone()
        }
    }
}

impl RayT {
    pub fn new(p: Pnt3f, d: Vec3f, t: Real) -> RayT {
        RayT {
            r: Ray::new(p, d),
            t,
        }
    }

    pub fn from_ray(r: Ray, t: Real) -> RayT {
        RayT { r, t }
    }

    pub fn is_nearer_than(&self, other: &RayT) -> bool {
        if self.t < 0.0 {
            false
        } else if other.t < 0.0 {
            true
        } else {
            self.t < other.t
        }
    }
}
