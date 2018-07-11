//! Triangle

use super::Ray;
use math::*;
use std::ops::Index;
use std::ops::IndexMut;

/// Ideal triangle model
#[derive(Clone)]
pub struct Triangle {
    vtx: [Vec3f; 3],
}

pub struct TriangleIntersection {
    pub t: Real,
    pub beta: Real,
    pub gamma: Real,
}

impl Index<usize> for Triangle {
    type Output = Vec3f;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);
        &self.vtx[index]
    }
}

impl IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 3);
        &mut self.vtx[index]
    }
}

impl Triangle {
    fn deter_a(&self, r: &Ray) -> Real {
        Mat3f::new(
            self.vtx[0].x - self.vtx[1].x,
            self.vtx[0].y - self.vtx[1].y,
            self.vtx[0].z - self.vtx[1].z,
            self.vtx[0].x - self.vtx[2].x,
            self.vtx[0].y - self.vtx[2].y,
            self.vtx[0].z - self.vtx[2].z,
            r.d.x,
            r.d.y,
            r.d.z,
        ).determinant()
    }

    fn deter_t(&self, r: &Ray) -> Real {
        Mat3f::new(
            self.vtx[0].x - self.vtx[1].x,
            self.vtx[0].y - self.vtx[1].y,
            self.vtx[0].z - self.vtx[1].z,
            self.vtx[0].x - self.vtx[2].x,
            self.vtx[0].y - self.vtx[2].y,
            self.vtx[0].z - self.vtx[2].z,
            self.vtx[0].x - r.p.x,
            self.vtx[0].y - r.p.y,
            self.vtx[0].z - r.p.z,
        ).determinant()
    }

    fn deter_beta(&self, r: &Ray) -> Real {
        Mat3f::new(
            self.vtx[0].x - r.p.x,
            self.vtx[0].y - r.p.y,
            self.vtx[0].z - r.p.z,
            self.vtx[0].x - self.vtx[2].x,
            self.vtx[0].y - self.vtx[2].y,
            self.vtx[0].z - self.vtx[2].z,
            r.d.x,
            r.d.y,
            r.d.z,
        ).determinant()
    }

    fn deter_gamma(&self, r: &Ray) -> Real {
        Mat3f::new(
            self.vtx[0].x - self.vtx[1].x,
            self.vtx[0].y - self.vtx[1].y,
            self.vtx[0].z - self.vtx[1].z,
            self.vtx[0].x - r.p.x,
            self.vtx[0].y - r.p.y,
            self.vtx[0].z - r.p.z,
            r.d.x,
            r.d.y,
            r.d.z,
        ).determinant()
    }

    pub fn new(vtx: [Vec3f; 3]) -> Triangle {
        Triangle { vtx }
    }

    pub fn new_abc(a: Vec3f, b: Vec3f, c: Vec3f) -> Triangle {
        Triangle { vtx: [a, b, c] }
    }

    pub fn is_intersected(&self, r: Ray) -> Option<Real> {
        let a = self.deter_a(&r);
        if a.relative_eq(&0.0, 1e-6, 1e-6) {
            return None;
        }

        let t = self.deter_t(&r) / a;
        if t <= 0.0 {
            return None;
        }

        let beta = self.deter_beta(&r) / a;
        if beta < 0.0 || beta > 1.0 {
            return None;
        }

        let gamma = self.deter_gamma(&r) / a;
        if gamma < 0.0 || gamma > 1.0 {
            return None;
        }

        Some(t)
    }

    pub fn nearest_inct(&self, r: Ray) -> Option<TriangleIntersection> {
        let a = self.deter_a(&r);
        if a.relative_eq(&0.0, 1e-6, 1e-6) {
            return None;
        }

        let t = self.deter_t(&r) / a;
        if t <= 0.0 {
            return None;
        }

        let beta = self.deter_beta(&r) / a;
        if beta < 0.0 || beta > 1.0 {
            return None;
        }

        let gamma = self.deter_gamma(&r) / a;
        if gamma < 0.0 || gamma > 1.0 {
            return None;
        }

        Some(TriangleIntersection { t, beta, gamma })
    }

    /// r: ray intersecting with self.
    /// cos<r.d, return value> shall be less than 0
    pub fn normal(&self, r: &Ray) -> Vec3f {
        let n = (self.vtx[1] - self.vtx[0])
            .cross(self.vtx[2] - self.vtx[0])
            .normalize();
        if dot(n, r.d) < 0.0 {
            n
        } else {
            -n
        }
    }
}
