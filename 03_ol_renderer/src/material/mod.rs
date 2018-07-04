//! Programming interfaces for object materials

use math::*;

pub enum BxDFType {
    /// Reflection only
    BRDF,
    /// Reflection + refraction
    BSDF,
}

pub trait BxDF {
    /// BxDF type. Returned value shall be consistent during the whole lifetime.
    fn get_type() -> BxDFType;

    /// Compute the BxDF coefficient.
    fn f(vin: &Vec3f, vout: &Vec3f) -> Color3f;

    /// Sample directions
    fn sample(v: &Vec3f, n: u32) -> Vec<Vec3f>;

    /// Probability density
    fn pdf(v: &Vec3f, vsample: &Vec3f) -> Real;
}
