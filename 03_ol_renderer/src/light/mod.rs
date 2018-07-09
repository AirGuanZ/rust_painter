//! Light sources

use math::*;

pub struct LightSample {
    pub ray: Ray,
    pub color: Color3f,
}

pub trait Light {
    fn sample(&self, n: u32) -> Vec<LightSample>;
    fn pdf(&self, ray: Ray) -> Real;

    fn sample_to(&self, n: u32, dst_pnt: Vec3f) -> Vec<LightSample>;
    fn pdf_to(&self, ray: Ray, dst_pnt: Vec3f) -> Real;
}
