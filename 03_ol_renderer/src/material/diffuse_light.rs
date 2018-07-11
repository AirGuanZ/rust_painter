//! Diffuse light source

use material::*;
use math::*;

pub struct DiffuseLight {
    normal: Vec3f,
    color: Color3f,
}

impl BxDF for DiffuseLight {
    fn get_type(&self) -> BxDFType {
        BxDFType::BRDF
    }

    fn ambient(&self) -> Color3f {
        BLACK
    }

    fn emit(&self, v: Vec3f) -> Color3f {
        if dot(self.normal, v) <= 0.0 {
            BLACK
        } else {
            self.color
        }
    }

    fn f(&self, _: Vec3f, _: Vec3f) -> Color3f {
        BLACK
    }

    fn sample(&self, _: &Vec3f, _: u32) -> Vec<Vec3f> {
        vec![]
    }

    fn pdf(&self, _: &Vec3f, _: &Vec3f) -> Real {
        0.0
    }

    fn sample_upper(&self, _: &Vec3f, _: u32) -> Vec<Vec3f> {
        vec![]
    }

    fn pdf_upper(&self, _: &Vec3f, _: &Vec3f) -> Real {
        0.0
    }
}

impl DiffuseLight {
    pub fn new(normal: Vec3f, color: Color3f) -> DiffuseLight {
        DiffuseLight { normal, color }
    }
}
