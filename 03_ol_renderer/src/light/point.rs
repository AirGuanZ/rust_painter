//! Point light source

extern crate rand;

use light::*;
use math::*;

pub struct PointLight {
    pos: Vec3f,
    color: Color3f,
}

impl Light for PointLight {
    fn sample(&self, n: u32) -> Vec<LightSample> {
        (0..n)
            .map(|_| LightSample {
                ray: Ray::new(self.pos, sphere_uniform()),
                color: self.color,
            })
            .collect()
    }

    fn pdf(&self, _ray: Ray) -> Real {
        1.0 / (4.0 * REAL_PI)
    }

    fn sample_to(&self, n: u32, dst_pnt: Vec3f) -> Vec<LightSample> {
        let dir = (dst_pnt - self.pos).normalize();
        (0..n)
            .map(|_| LightSample {
                ray: Ray::new(self.pos, dir),
                color: self.color,
            })
            .collect()
    }

    fn pdf_to(&self, _ray: Ray, _dst_pnt: Vec3f) -> Real {
        1.0
    }
}

impl PointLight {
    pub fn new(pos: Vec3f, color: Color3f) -> PointLight {
        PointLight { pos, color }
    }
}
