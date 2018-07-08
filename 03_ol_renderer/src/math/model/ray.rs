//! Implementation of p = p0 + t * d

use std::ops::Mul;
use math::{mat::*, Clamp, Real};

#[derive(Clone)]
pub struct Ray {
    pub p: Vec3f,
    pub d: Vec3f,
}

#[derive(Clone)]
pub struct RayT {
    pub r: Ray,
    pub t: Real,
}

impl Mul<Ray> for Mat4f {
    type Output = Ray;

    fn mul(self, r: Ray) -> Ray {
        Ray::new(
            (self * vec4(r.p.x, r.p.y, r.p.z, 1.0)).xyz(),
            (self * vec4(r.d.x, r.d.y, r.d.z, 0.0)).xyz(),
        )
    }
}

impl Ray {
    pub fn new(p: Vec3f, d: Vec3f) -> Ray {
        Ray {
            p,
            d: d.normalize(),
        }
    }

    pub fn t_to_point(&self, t: Real) -> Vec3f {
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
    pub fn new(p: Vec3f, d: Vec3f, t: Real) -> RayT {
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
