//! Programming interfaces for object materials

pub mod combine;
pub mod phong;

pub mod prelude {
    pub use super::combine::*;
    pub use super::phong::*;
    use math::*;

    #[derive(Clone, PartialEq, Eq)]
    pub enum BxDFType {
        /// Reflection only
        BRDF,
        /// Reflection + refraction
        BSDF,
    }

    pub trait BxDF {
        /// BxDF type. Returned value shall be consistent during the whole lifetime.
        fn get_type(&self) -> BxDFType;

        /// Ambient illumination
        fn ambient(&self) -> Color3f;

        /// Compute the BxDF coefficient.
        fn f(&self, vin: Vec3f, vout: Vec3f) -> Color3f;

        /// Sample directions
        fn sample(&self, v: &Vec3f, n: u32) -> Vec<Vec3f>;

        /// Probability density
        fn pdf(&self, v: &Vec3f, vsample: &Vec3f) -> Real;

        /// (For Combinator) Probability of sampling on upper hemisphere
        fn sample_upper(&self, v: &Vec3f, n: u32) -> Vec<Vec3f> {
            self.sample(v, n)
        }

        /// (For Combinator) Probability density on upper hemisphere
        fn pdf_upper(&self, v: &Vec3f, vsample: &Vec3f) -> Real {
            self.pdf(v, vsample)
        }
    }
}

pub use self::prelude::*;
