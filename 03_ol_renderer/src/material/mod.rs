//! Programming interfaces for object materials

pub mod cook_torrance;
pub mod fresnel;
pub mod metal;

pub mod prelude {
    pub use super::cook_torrance::*;
    pub use super::metal::*;
}

pub use self::prelude::*;
use math::*;

pub enum BxDFType {
    /// Reflection only
    BRDF,
    /// Reflection + refraction
    BSDF,
}

pub trait BxDF {
    /// BxDF type. Returned value shall be consistent during the whole lifetime.
    fn get_type(&self) -> BxDFType;

    /// Compute the BxDF coefficient.
    fn f(&self, vin: &Vec3f, vout: &Vec3f) -> Color3f;

    /// Sample directions
    fn sample(&self, v: &Vec3f, n: u32) -> Vec<Vec3f>;

    /// Probability density
    fn pdf(&self, v: &Vec3f, vsample: &Vec3f) -> Real;
}
