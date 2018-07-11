//! Light sources

pub mod point;

pub mod light {}

pub mod prelude {
    pub use super::point::*;
    use math::*;

    pub struct LightSample {
        pub light_normal: Vec3f,
        pub ray: Ray,
        pub color: Color3f,
    }

    pub trait Light: Sync {
        fn sample(&self, n: u32) -> Vec<LightSample>;
        fn pdf(&self, ray: Ray) -> Real;

        fn sample_to(&self, n: u32, dst_pnt: Vec3f) -> Vec<LightSample>;
        fn pdf_to(&self, ray: Ray, dst_pnt: Vec3f) -> Real;
    }
}

pub use self::prelude::*;
