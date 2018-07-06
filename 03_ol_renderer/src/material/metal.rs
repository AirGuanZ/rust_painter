//! Solid metal surface

use super::*;

pub struct Metal {

}

impl BxDF for Metal {
    fn get_type(&self) -> BxDFType {
        BxDFType::BRDF
    }

    fn f(&self, vin: &Vec3f, vout: &Vec3f) -> Color3f {
        color3(1.0, 1.0, 1.0)
    }

    fn sample(&self, v: &Vec3f, n: u32) -> Vec<Vec3f> {
        vec![]
    }

    fn pdf(&self, v: &Vec3f, vsample: &Vec3f) -> Real {
        1.0
    }
}
